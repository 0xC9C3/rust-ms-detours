#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unaligned_references)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::ffi::{CStr, CString};
    use std::ptr;
    use super::*;

    #[test]
    fn test_find_function()
    {
        unsafe {
            let module = CString::new("kernel32.dll").unwrap();
            let function = CString::new("Sleep").unwrap();
            let detours_p = DetourFindFunction(module.as_ptr(), function.as_ptr());

            let kernel_dll_handle = GetModuleHandleA(module.as_ptr());
            let raw_p = GetProcAddress(kernel_dll_handle, function.as_ptr());

            assert!(raw_p.is_some());

            let raw_p = raw_p.unwrap();
            assert_eq!(detours_p as usize, raw_p as usize);
        }
    }

    #[test]
    fn test_enumerate_modules()
    {
        unsafe {
            let h_module = DetourEnumerateModules(ptr::null_mut());
            assert_ne!(h_module as usize, 0);
        }
    }

    #[test]
    fn test_enumerate_exports()
    {
        unsafe {
            // skip entry module
            let h_module = DetourEnumerateModules(ptr::null_mut());
            let h_module = DetourEnumerateModules(h_module);
            let success = DetourEnumerateExports(h_module, ptr::null_mut(), Some(exports_cb));
            assert!(success > 0);
        }
    }

    unsafe extern "C" fn exports_cb(pContext: PVOID, nOrdinal: ULONG, pszName: LPCSTR, pCode: PVOID) -> BOOL
    {
        println!("pContext {:#?}", pContext);
        println!("nOrdinal {:#?}", nOrdinal);
        println!("pszName {:#?}", CStr::from_ptr(pszName));
        println!("pCode {:#?}", pCode);
        return 1;
    }
}