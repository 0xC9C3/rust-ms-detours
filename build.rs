extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("msbuild /p:Configuration=ReleaseMD /p:Platform=x64");

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=detours\\lib.X64");

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
        .header("wrapper.h")
        .blocklist_type("LPMONITORINFOEXA?W?")
        .blocklist_type("LPTOP_LEVEL_EXCEPTION_FILTER")
        .blocklist_type("MONITORINFOEXA?W?")
        .blocklist_type("PEXCEPTION_FILTER")
        .blocklist_type("PEXCEPTION_ROUTINE")
        .blocklist_type("PSLIST_HEADER")
        .blocklist_type("PTOP_LEVEL_EXCEPTION_FILTER")
        .blocklist_type("PVECTORED_EXCEPTION_HANDLER")
        .blocklist_type("_?L?P?CONTEXT")
        .blocklist_type("_?L?P?EXCEPTION_POINTERS")
        .blocklist_type("_?P?DISPATCHER_CONTEXT")
        .blocklist_type("_?P?EXCEPTION_REGISTRATION_RECORD")
        .blocklist_type("_?P?IMAGE_TLS_DIRECTORY.*")
        .blocklist_type("_?P?NT_TIB")
        .blocklist_type("tagMONITORINFOEXA")
        .blocklist_type("tagMONITORINFOEXW")
        .blocklist_function("AddVectoredContinueHandler")
        .blocklist_function("AddVectoredExceptionHandler")
        .blocklist_function("CopyContext")
        .blocklist_function("GetThreadContext")
        .blocklist_function("GetXStateFeaturesMask")
        .blocklist_function("InitializeContext")
        .blocklist_function("InitializeContext2")
        .blocklist_function("InitializeSListHead")
        .blocklist_function("InterlockedFlushSList")
        .blocklist_function("InterlockedPopEntrySList")
        .blocklist_function("InterlockedPushEntrySList")
        .blocklist_function("InterlockedPushListSListEx")
        .blocklist_function("LocateXStateFeature")
        .blocklist_function("QueryDepthSList")
        .blocklist_function("RaiseFailFastException")
        .blocklist_function("RtlCaptureContext")
        .blocklist_function("RtlCaptureContext2")
        .blocklist_function("RtlFirstEntrySList")
        .blocklist_function("RtlInitializeSListHead")
        .blocklist_function("RtlInterlockedFlushSList")
        .blocklist_function("RtlInterlockedPopEntrySList")
        .blocklist_function("RtlInterlockedPushEntrySList")
        .blocklist_function("RtlInterlockedPushListSListEx")
        .blocklist_function("RtlQueryDepthSList")
        .blocklist_function("RtlRestoreContext")
        .blocklist_function("RtlUnwindEx")
        .blocklist_function("RtlVirtualUnwind")
        .blocklist_function("SetThreadContext")
        .blocklist_function("SetUnhandledExceptionFilter")
        .blocklist_function("SetXStateFeaturesMask")
        .blocklist_function("UnhandledExceptionFilter")
        .blocklist_function("__C_specific_handler")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
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