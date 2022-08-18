extern crate bindgen;
extern crate core;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use cc::windows_registry::find_tool;

// adopted and modified from https://github.com/compass-rs/sass-rs/blob/master/sass-sys/build.rs


macro_rules! t {
    ($e:expr) => (match $e {
        Ok(n) => n,
        Err(e) => panic!("\n{} failed with {}\n", stringify!($e), e),
    })
}

fn main() {
    let src = get_detours_folder();
    let tool = find_tool("msvc", "msbuild").unwrap();

    let target = env::var("TARGET").expect("TARGET not found in environment");

    let mut msvc_platform = if target.contains("x86_64") {
        "x64"
    } else {
        "x86"
    };

    if target.starts_with("aarch64") {
        msvc_platform = "ARM64";
    }

    if target.starts_with("arm") {
        msvc_platform = "ARM";
    }

    let dest = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR not found in environment"));
    let build = dest.join("build");

    t!(fs::create_dir_all(&build));
    cp_r(&src, &build);


    fs::copy(&env::current_dir().unwrap().join("wrapper.h"), &build.join("wrapper.h")).unwrap();

    tool.to_command()
        .current_dir(&build)
        .args(
        [
            "vc\\Detours.sln",
            "/p:Configuration=ReleaseMD",
            format!("/p:Platform={}", msvc_platform).as_str(),
        ]
    )
        .status()
        .unwrap();

    // Tell cargo to look for shared libraries in the specified directory
    let target_folder = format!("lib.{}", msvc_platform);
    println!("cargo:rustc-link-search={}", build.join(target_folder).display());

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=detours");
    println!("cargo:rustc-link-lib=syelog");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(build.join("wrapper.h").to_str().unwrap())
        .allowlist_function("DetourTransactionBegin")
        .allowlist_function("DetourUpdateThread")
        .allowlist_function("DetourAttach")
        .allowlist_function("DetourAttachEx")
        .allowlist_function("DetourAllocateRegionWithinJumpBounds")
        .allowlist_function("DetourDetach")
        .allowlist_function("DetourSetIgnoreTooSmall")
        .allowlist_function("DetourSetRetainRegions")
        .allowlist_function("DetourSetSystemRegionLowerBound")
        .allowlist_function("DetourSetSystemRegionUpperBound")
        .allowlist_function("DetourTransactionAbort")
        .allowlist_function("DetourTransactionCommit")
        .allowlist_function("DetourTransactionCommitEx")
        .allowlist_function("DetourFindFunction")
        .allowlist_function("DetourCodeFromPointer")
        .allowlist_function("DetourEnumerateModules")
        .allowlist_function("DetourGetEntryPoint")
        .allowlist_function("DetourGetModuleSize")
        .allowlist_function("DetourEnumerateExports")
        .allowlist_function("DetourEnumerateImport")
        .allowlist_function("DetourEnumerateImportEx")
        .allowlist_function("DetourFindPayload")
        .allowlist_function("DetourFindPayloadEx")
        .allowlist_function("DetourFindRemotePayload")
        .allowlist_function("DetourGetContainingModule")
        .allowlist_function("DetourGetSizeOfPayloads")
        .allowlist_function("DetourBinaryOpen")
        .allowlist_function("DetourBinaryEnumeratePayloads")
        .allowlist_function("DetourBinaryFindPayload")
        .allowlist_function("DetourBinarySetPayload")
        .allowlist_function("DetourBinaryDeletePayload")
        .allowlist_function("DetourBinaryPurgePayloads")
        .allowlist_function("DetourBinaryEditImports")
        .allowlist_function("DetourBinaryResetImports")
        .allowlist_function("DetourBinaryWrite")
        .allowlist_function("DetourBinaryClose")
        .allowlist_function("DetourCreateProcessWithDllEx")
        .allowlist_function("DetourCreateProcessWithDlls")
        .allowlist_function("DetourCopyPayloadToProcess")
        .allowlist_function("DetourCopyPayloadToProcessEx")
        .allowlist_function("DetourFinishHelperProcess")
        .allowlist_function("DetourIsHelperProcess")
        .allowlist_function("DetourRestoreAfterWith")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .layout_tests(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn cp_r(dir: &Path, dest: &Path) {
    for entry in t!(fs::read_dir(dir)) {
        let entry = t!(entry);
        let path = entry.path();
        let dst = dest.join(path.file_name().expect("Failed to get filename of path"));
        if t!(fs::metadata(&path)).is_file() {
            t!(fs::copy(path, dst));
        } else {
            t!(fs::create_dir_all(&dst));
            cp_r(&path, &dst);
        }
    }
}

fn get_detours_folder() -> PathBuf {
    env::current_dir().expect("Failed to get the current directory").join("detours")
}