/* automatically generated by rust-bindgen 0.60.1 */

pub type wchar_t = ::std::os::raw::c_ushort;
pub type ULONG = ::std::os::raw::c_ulong;
pub type DWORD = ::std::os::raw::c_ulong;
pub type BOOL = ::std::os::raw::c_int;
pub type BYTE = ::std::os::raw::c_uchar;
pub type WORD = ::std::os::raw::c_ushort;
pub type PBYTE = *mut BYTE;
pub type LPBYTE = *mut BYTE;
pub type PDWORD = *mut DWORD;
pub type LPVOID = *mut ::std::os::raw::c_void;
pub type LPCVOID = *const ::std::os::raw::c_void;
pub type INT = ::std::os::raw::c_int;
pub type PVOID = *mut ::std::os::raw::c_void;
pub type CHAR = ::std::os::raw::c_char;
pub type LONG = ::std::os::raw::c_long;
pub type WCHAR = wchar_t;
pub type LPWSTR = *mut WCHAR;
pub type LPCWSTR = *const WCHAR;
pub type LPSTR = *mut CHAR;
pub type LPCSTR = *const CHAR;
pub type HANDLE = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GUID {
    pub Data1: ::std::os::raw::c_ulong,
    pub Data2: ::std::os::raw::c_ushort,
    pub Data3: ::std::os::raw::c_ushort,
    pub Data4: [::std::os::raw::c_uchar; 8usize],
}
pub type GUID = _GUID;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HINSTANCE__ {
    pub unused: ::std::os::raw::c_int,
}
pub type HINSTANCE = *mut HINSTANCE__;
pub type HMODULE = HINSTANCE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HWND__ {
    pub unused: ::std::os::raw::c_int,
}
pub type HWND = *mut HWND__;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _SECURITY_ATTRIBUTES {
    pub nLength: DWORD,
    pub lpSecurityDescriptor: LPVOID,
    pub bInheritHandle: BOOL,
}
pub type LPSECURITY_ATTRIBUTES = *mut _SECURITY_ATTRIBUTES;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PROCESS_INFORMATION {
    pub hProcess: HANDLE,
    pub hThread: HANDLE,
    pub dwProcessId: DWORD,
    pub dwThreadId: DWORD,
}
pub type LPPROCESS_INFORMATION = *mut _PROCESS_INFORMATION;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _STARTUPINFOA {
    pub cb: DWORD,
    pub lpReserved: LPSTR,
    pub lpDesktop: LPSTR,
    pub lpTitle: LPSTR,
    pub dwX: DWORD,
    pub dwY: DWORD,
    pub dwXSize: DWORD,
    pub dwYSize: DWORD,
    pub dwXCountChars: DWORD,
    pub dwYCountChars: DWORD,
    pub dwFillAttribute: DWORD,
    pub dwFlags: DWORD,
    pub wShowWindow: WORD,
    pub cbReserved2: WORD,
    pub lpReserved2: LPBYTE,
    pub hStdInput: HANDLE,
    pub hStdOutput: HANDLE,
    pub hStdError: HANDLE,
}
pub type LPSTARTUPINFOA = *mut _STARTUPINFOA;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _STARTUPINFOW {
    pub cb: DWORD,
    pub lpReserved: LPWSTR,
    pub lpDesktop: LPWSTR,
    pub lpTitle: LPWSTR,
    pub dwX: DWORD,
    pub dwY: DWORD,
    pub dwXSize: DWORD,
    pub dwYSize: DWORD,
    pub dwXCountChars: DWORD,
    pub dwYCountChars: DWORD,
    pub dwFillAttribute: DWORD,
    pub dwFlags: DWORD,
    pub wShowWindow: WORD,
    pub cbReserved2: WORD,
    pub lpReserved2: LPBYTE,
    pub hStdInput: HANDLE,
    pub hStdOutput: HANDLE,
    pub hStdError: HANDLE,
}
pub type LPSTARTUPINFOW = *mut _STARTUPINFOW;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _DETOUR_TRAMPOLINE {
    _unused: [u8; 0],
}
pub type PDETOUR_TRAMPOLINE = *mut _DETOUR_TRAMPOLINE;
#[doc = " Binary Typedefs."]
pub type PF_DETOUR_BINARY_BYWAY_CALLBACK = ::std::option::Option<
    unsafe extern "C" fn(pContext: PVOID, pszFile: LPCSTR, ppszOutFile: *mut LPCSTR) -> BOOL,
>;
pub type PF_DETOUR_BINARY_FILE_CALLBACK = ::std::option::Option<
    unsafe extern "C" fn(
        pContext: PVOID,
        pszOrigFile: LPCSTR,
        pszFile: LPCSTR,
        ppszOutFile: *mut LPCSTR,
    ) -> BOOL,
>;
pub type PF_DETOUR_BINARY_SYMBOL_CALLBACK = ::std::option::Option<
    unsafe extern "C" fn(
        pContext: PVOID,
        nOrigOrdinal: ULONG,
        nOrdinal: ULONG,
        pnOutOrdinal: *mut ULONG,
        pszOrigSymbol: LPCSTR,
        pszSymbol: LPCSTR,
        ppszOutSymbol: *mut LPCSTR,
    ) -> BOOL,
>;
pub type PF_DETOUR_BINARY_COMMIT_CALLBACK =
    ::std::option::Option<unsafe extern "C" fn(pContext: PVOID) -> BOOL>;
pub type PF_DETOUR_ENUMERATE_EXPORT_CALLBACK = ::std::option::Option<
    unsafe extern "C" fn(pContext: PVOID, nOrdinal: ULONG, pszName: LPCSTR, pCode: PVOID) -> BOOL,
>;
pub type PF_DETOUR_IMPORT_FILE_CALLBACK = ::std::option::Option<
    unsafe extern "C" fn(pContext: PVOID, hModule: HMODULE, pszFile: LPCSTR) -> BOOL,
>;
pub type PF_DETOUR_IMPORT_FUNC_CALLBACK = ::std::option::Option<
    unsafe extern "C" fn(pContext: PVOID, nOrdinal: DWORD, pszFunc: LPCSTR, pvFunc: PVOID) -> BOOL,
>;
pub type PF_DETOUR_IMPORT_FUNC_CALLBACK_EX = ::std::option::Option<
    unsafe extern "C" fn(
        pContext: PVOID,
        nOrdinal: DWORD,
        pszFunc: LPCSTR,
        ppvFunc: *mut PVOID,
    ) -> BOOL,
>;
pub type PDETOUR_BINARY = *mut ::std::os::raw::c_void;
extern "C" {
    #[doc = " Transaction APIs."]
    pub fn DetourTransactionBegin() -> LONG;
}
extern "C" {
    pub fn DetourTransactionAbort() -> LONG;
}
extern "C" {
    pub fn DetourTransactionCommit() -> LONG;
}
extern "C" {
    pub fn DetourTransactionCommitEx(pppFailedPointer: *mut *mut PVOID) -> LONG;
}
extern "C" {
    pub fn DetourUpdateThread(hThread: HANDLE) -> LONG;
}
extern "C" {
    pub fn DetourAttach(ppPointer: *mut PVOID, pDetour: PVOID) -> LONG;
}
extern "C" {
    pub fn DetourAttachEx(
        ppPointer: *mut PVOID,
        pDetour: PVOID,
        ppRealTrampoline: *mut PDETOUR_TRAMPOLINE,
        ppRealTarget: *mut PVOID,
        ppRealDetour: *mut PVOID,
    ) -> LONG;
}
extern "C" {
    pub fn DetourDetach(ppPointer: *mut PVOID, pDetour: PVOID) -> LONG;
}
extern "C" {
    pub fn DetourSetIgnoreTooSmall(fIgnore: BOOL) -> BOOL;
}
extern "C" {
    pub fn DetourSetRetainRegions(fRetain: BOOL) -> BOOL;
}
extern "C" {
    pub fn DetourSetSystemRegionLowerBound(pSystemRegionLowerBound: PVOID) -> PVOID;
}
extern "C" {
    pub fn DetourSetSystemRegionUpperBound(pSystemRegionUpperBound: PVOID) -> PVOID;
}
extern "C" {
    #[doc = " Code Functions."]
    pub fn DetourFindFunction(pszModule: LPCSTR, pszFunction: LPCSTR) -> PVOID;
}
extern "C" {
    pub fn DetourCodeFromPointer(pPointer: PVOID, ppGlobals: *mut PVOID) -> PVOID;
}
extern "C" {
    pub fn DetourCopyInstruction(
        pDst: PVOID,
        ppDstPool: *mut PVOID,
        pSrc: PVOID,
        ppTarget: *mut PVOID,
        plExtra: *mut LONG,
    ) -> PVOID;
}
extern "C" {
    pub fn DetourSetCodeModule(hModule: HMODULE, fLimitReferencesToModule: BOOL) -> BOOL;
}
extern "C" {
    pub fn DetourAllocateRegionWithinJumpBounds(
        pbTarget: LPCVOID,
        pcbAllocatedSize: PDWORD,
    ) -> PVOID;
}
extern "C" {
    pub fn DetourIsFunctionImported(pbCode: PBYTE, pbAddress: PBYTE) -> BOOL;
}
extern "C" {
    #[doc = " Loaded Binary Functions."]
    pub fn DetourGetContainingModule(pvAddr: PVOID) -> HMODULE;
}
extern "C" {
    pub fn DetourEnumerateModules(hModuleLast: HMODULE) -> HMODULE;
}
extern "C" {
    pub fn DetourGetEntryPoint(hModule: HMODULE) -> PVOID;
}
extern "C" {
    pub fn DetourGetModuleSize(hModule: HMODULE) -> ULONG;
}
extern "C" {
    pub fn DetourEnumerateExports(
        hModule: HMODULE,
        pContext: PVOID,
        pfExport: PF_DETOUR_ENUMERATE_EXPORT_CALLBACK,
    ) -> BOOL;
}
extern "C" {
    pub fn DetourEnumerateImports(
        hModule: HMODULE,
        pContext: PVOID,
        pfImportFile: PF_DETOUR_IMPORT_FILE_CALLBACK,
        pfImportFunc: PF_DETOUR_IMPORT_FUNC_CALLBACK,
    ) -> BOOL;
}
extern "C" {
    pub fn DetourEnumerateImportsEx(
        hModule: HMODULE,
        pContext: PVOID,
        pfImportFile: PF_DETOUR_IMPORT_FILE_CALLBACK,
        pfImportFuncEx: PF_DETOUR_IMPORT_FUNC_CALLBACK_EX,
    ) -> BOOL;
}
extern "C" {
    pub fn DetourFindPayload(hModule: HMODULE, rguid: *const GUID, pcbData: *mut DWORD) -> PVOID;
}
extern "C" {
    pub fn DetourFindPayloadEx(rguid: *const GUID, pcbData: *mut DWORD) -> PVOID;
}
extern "C" {
    pub fn DetourGetSizeOfPayloads(hModule: HMODULE) -> DWORD;
}
extern "C" {
    pub fn DetourFreePayload(pvData: PVOID) -> BOOL;
}
extern "C" {
    #[doc = " Persistent Binary Functions."]
    pub fn DetourBinaryOpen(hFile: HANDLE) -> PDETOUR_BINARY;
}
extern "C" {
    pub fn DetourBinaryEnumeratePayloads(
        pBinary: PDETOUR_BINARY,
        pGuid: *mut GUID,
        pcbData: *mut DWORD,
        pnIterator: *mut DWORD,
    ) -> PVOID;
}
extern "C" {
    pub fn DetourBinaryFindPayload(
        pBinary: PDETOUR_BINARY,
        rguid: *const GUID,
        pcbData: *mut DWORD,
    ) -> PVOID;
}
extern "C" {
    pub fn DetourBinarySetPayload(
        pBinary: PDETOUR_BINARY,
        rguid: *const GUID,
        pData: PVOID,
        cbData: DWORD,
    ) -> PVOID;
}
extern "C" {
    pub fn DetourBinaryDeletePayload(pBinary: PDETOUR_BINARY, rguid: *const GUID) -> BOOL;
}
extern "C" {
    pub fn DetourBinaryPurgePayloads(pBinary: PDETOUR_BINARY) -> BOOL;
}
extern "C" {
    pub fn DetourBinaryResetImports(pBinary: PDETOUR_BINARY) -> BOOL;
}
extern "C" {
    pub fn DetourBinaryEditImports(
        pBinary: PDETOUR_BINARY,
        pContext: PVOID,
        pfByway: PF_DETOUR_BINARY_BYWAY_CALLBACK,
        pfFile: PF_DETOUR_BINARY_FILE_CALLBACK,
        pfSymbol: PF_DETOUR_BINARY_SYMBOL_CALLBACK,
        pfCommit: PF_DETOUR_BINARY_COMMIT_CALLBACK,
    ) -> BOOL;
}
extern "C" {
    pub fn DetourBinaryWrite(pBinary: PDETOUR_BINARY, hFile: HANDLE) -> BOOL;
}
extern "C" {
    pub fn DetourBinaryClose(pBinary: PDETOUR_BINARY) -> BOOL;
}
extern "C" {
    #[doc = " Create Process & Load Dll."]
    pub fn DetourFindRemotePayload(
        hProcess: HANDLE,
        rguid: *const GUID,
        pcbData: *mut DWORD,
    ) -> PVOID;
}
pub type PDETOUR_CREATE_PROCESS_ROUTINEA = ::std::option::Option<
    unsafe extern "C" fn(
        lpApplicationName: LPCSTR,
        lpCommandLine: LPSTR,
        lpProcessAttributes: LPSECURITY_ATTRIBUTES,
        lpThreadAttributes: LPSECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: DWORD,
        lpEnvironment: LPVOID,
        lpCurrentDirectory: LPCSTR,
        lpStartupInfo: LPSTARTUPINFOA,
        lpProcessInformation: LPPROCESS_INFORMATION,
    ) -> BOOL,
>;
pub type PDETOUR_CREATE_PROCESS_ROUTINEW = ::std::option::Option<
    unsafe extern "C" fn(
        lpApplicationName: LPCWSTR,
        lpCommandLine: LPWSTR,
        lpProcessAttributes: LPSECURITY_ATTRIBUTES,
        lpThreadAttributes: LPSECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: DWORD,
        lpEnvironment: LPVOID,
        lpCurrentDirectory: LPCWSTR,
        lpStartupInfo: LPSTARTUPINFOW,
        lpProcessInformation: LPPROCESS_INFORMATION,
    ) -> BOOL,
>;
extern "C" {
    pub fn DetourCreateProcessWithDllA(
        lpApplicationName: LPCSTR,
        lpCommandLine: LPSTR,
        lpProcessAttributes: LPSECURITY_ATTRIBUTES,
        lpThreadAttributes: LPSECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: DWORD,
        lpEnvironment: LPVOID,
        lpCurrentDirectory: LPCSTR,
        lpStartupInfo: LPSTARTUPINFOA,
        lpProcessInformation: LPPROCESS_INFORMATION,
        lpDllName: LPCSTR,
        pfCreateProcessA: PDETOUR_CREATE_PROCESS_ROUTINEA,
    ) -> BOOL;
}
extern "C" {
    pub fn DetourCreateProcessWithDllW(
        lpApplicationName: LPCWSTR,
        lpCommandLine: LPWSTR,
        lpProcessAttributes: LPSECURITY_ATTRIBUTES,
        lpThreadAttributes: LPSECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: DWORD,
        lpEnvironment: LPVOID,
        lpCurrentDirectory: LPCWSTR,
        lpStartupInfo: LPSTARTUPINFOW,
        lpProcessInformation: LPPROCESS_INFORMATION,
        lpDllName: LPCSTR,
        pfCreateProcessW: PDETOUR_CREATE_PROCESS_ROUTINEW,
    ) -> BOOL;
}
extern "C" {
    pub fn DetourCreateProcessWithDllExA(
        lpApplicationName: LPCSTR,
        lpCommandLine: LPSTR,
        lpProcessAttributes: LPSECURITY_ATTRIBUTES,
        lpThreadAttributes: LPSECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: DWORD,
        lpEnvironment: LPVOID,
        lpCurrentDirectory: LPCSTR,
        lpStartupInfo: LPSTARTUPINFOA,
        lpProcessInformation: LPPROCESS_INFORMATION,
        lpDllName: LPCSTR,
        pfCreateProcessA: PDETOUR_CREATE_PROCESS_ROUTINEA,
    ) -> BOOL;
}
extern "C" {
    pub fn DetourCreateProcessWithDllExW(
        lpApplicationName: LPCWSTR,
        lpCommandLine: LPWSTR,
        lpProcessAttributes: LPSECURITY_ATTRIBUTES,
        lpThreadAttributes: LPSECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: DWORD,
        lpEnvironment: LPVOID,
        lpCurrentDirectory: LPCWSTR,
        lpStartupInfo: LPSTARTUPINFOW,
        lpProcessInformation: LPPROCESS_INFORMATION,
        lpDllName: LPCSTR,
        pfCreateProcessW: PDETOUR_CREATE_PROCESS_ROUTINEW,
    ) -> BOOL;
}
extern "C" {
    pub fn DetourCreateProcessWithDllsA(
        lpApplicationName: LPCSTR,
        lpCommandLine: LPSTR,
        lpProcessAttributes: LPSECURITY_ATTRIBUTES,
        lpThreadAttributes: LPSECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: DWORD,
        lpEnvironment: LPVOID,
        lpCurrentDirectory: LPCSTR,
        lpStartupInfo: LPSTARTUPINFOA,
        lpProcessInformation: LPPROCESS_INFORMATION,
        nDlls: DWORD,
        rlpDlls: *mut LPCSTR,
        pfCreateProcessA: PDETOUR_CREATE_PROCESS_ROUTINEA,
    ) -> BOOL;
}
extern "C" {
    pub fn DetourCreateProcessWithDllsW(
        lpApplicationName: LPCWSTR,
        lpCommandLine: LPWSTR,
        lpProcessAttributes: LPSECURITY_ATTRIBUTES,
        lpThreadAttributes: LPSECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: DWORD,
        lpEnvironment: LPVOID,
        lpCurrentDirectory: LPCWSTR,
        lpStartupInfo: LPSTARTUPINFOW,
        lpProcessInformation: LPPROCESS_INFORMATION,
        nDlls: DWORD,
        rlpDlls: *mut LPCSTR,
        pfCreateProcessW: PDETOUR_CREATE_PROCESS_ROUTINEW,
    ) -> BOOL;
}
extern "C" {
    pub fn DetourProcessViaHelperA(
        dwTargetPid: DWORD,
        lpDllName: LPCSTR,
        pfCreateProcessA: PDETOUR_CREATE_PROCESS_ROUTINEA,
    ) -> BOOL;
}
extern "C" {
    pub fn DetourProcessViaHelperW(
        dwTargetPid: DWORD,
        lpDllName: LPCSTR,
        pfCreateProcessW: PDETOUR_CREATE_PROCESS_ROUTINEW,
    ) -> BOOL;
}
extern "C" {
    pub fn DetourProcessViaHelperDllsA(
        dwTargetPid: DWORD,
        nDlls: DWORD,
        rlpDlls: *mut LPCSTR,
        pfCreateProcessA: PDETOUR_CREATE_PROCESS_ROUTINEA,
    ) -> BOOL;
}
extern "C" {
    pub fn DetourProcessViaHelperDllsW(
        dwTargetPid: DWORD,
        nDlls: DWORD,
        rlpDlls: *mut LPCSTR,
        pfCreateProcessW: PDETOUR_CREATE_PROCESS_ROUTINEW,
    ) -> BOOL;
}
extern "C" {
    pub fn DetourUpdateProcessWithDll(hProcess: HANDLE, rlpDlls: *mut LPCSTR, nDlls: DWORD)
        -> BOOL;
}
extern "C" {
    pub fn DetourUpdateProcessWithDllEx(
        hProcess: HANDLE,
        hImage: HMODULE,
        bIs32Bit: BOOL,
        rlpDlls: *mut LPCSTR,
        nDlls: DWORD,
    ) -> BOOL;
}
extern "C" {
    pub fn DetourCopyPayloadToProcess(
        hProcess: HANDLE,
        rguid: *const GUID,
        pvData: LPCVOID,
        cbData: DWORD,
    ) -> BOOL;
}
extern "C" {
    pub fn DetourCopyPayloadToProcessEx(
        hProcess: HANDLE,
        rguid: *const GUID,
        pvData: LPCVOID,
        cbData: DWORD,
    ) -> PVOID;
}
extern "C" {
    pub fn DetourRestoreAfterWith() -> BOOL;
}
extern "C" {
    pub fn DetourRestoreAfterWithEx(pvData: PVOID, cbData: DWORD) -> BOOL;
}
extern "C" {
    pub fn DetourIsHelperProcess() -> BOOL;
}
extern "C" {
    pub fn DetourFinishHelperProcess(arg1: HWND, arg2: HINSTANCE, arg3: LPSTR, arg4: INT);
}