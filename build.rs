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

    let tool = find_tool("x86_64-pc-windows-msvc", "msbuild").unwrap();

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

    let result = tool.to_command()
        .current_dir(&build)
        .args(
            [
                "vc\\Detours.sln",
                "/p:Configuration=ReleaseMD",
                format!("/p:Platform={}", msvc_platform).as_str(),
            ]
        )
        .output()
        .unwrap();

    if !result.status.success() {
        println!("cargo:warning=msbuild result: {:?}", result);
    }

    // Tell cargo to look for shared libraries in the specified directory
    let target_folder = format!("lib.{}", msvc_platform);
    println!("cargo:rustc-link-search={}", build.join(target_folder).display());

    println!("cargo:rustc-link-lib=detours");
    println!("cargo:rustc-link-lib=syelog");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    //generate_bindings(build);
}

fn generate_bindings(build: PathBuf) {

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(build.join("wrapper.h").to_str().unwrap())
        .allowlist_function("Detour.*")
        .blocklist_type("LP.*")
        .blocklist_type("_GUID")
        .blocklist_type("GUID")
        .blocklist_type("ULONG")
        .blocklist_type("PVOID")
        .blocklist_type("DWORD")
        .blocklist_type("wchar_t")
        .blocklist_type("BOOL")
        .blocklist_type("BYTE")
        .blocklist_type("WORD")
        .blocklist_type("PBYTE")
        .blocklist_type("PDWORD")
        .blocklist_type("INT")
        .blocklist_type("CHAR")
        .blocklist_type("LONG")
        .blocklist_type("WCHAR")
        .blocklist_type("HANDLE")
        .blocklist_type("HMODULE")
        .blocklist_type("HINSTANCE.*")
        .blocklist_type("HWND.*")
        .blocklist_type("_SECURITY_ATTRIBUTES")
        .blocklist_type("_PROCESS_INFORMATION")
        .blocklist_type("_STARTUPINFOA")
        .blocklist_type("_STARTUPINFOW")
        .raw_line("use winapi::shared::minwindef::*;")
        .raw_line("use winapi::um::winnt::*;")
        .raw_line("use winapi::um::winnt::{INT};")
        .raw_line("use winapi::um::minwinbase::*;")
        .raw_line("use winapi::um::processthreadsapi::*;")
        .raw_line("use winapi::shared::guiddef::*;")
        .raw_line("use winapi::shared::windef::*;")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .layout_tests(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("src");
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
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("detours")
}