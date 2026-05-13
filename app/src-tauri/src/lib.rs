use windows::Win32::System::LibraryLoader::{LoadLibraryA, GetProcAddress};
use windows::Win32::Foundation::{FARPROC};
use windows::core::PCSTR;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;



type LdRunFlowFn = unsafe extern "C" fn(*const c_char) -> *const c_char;

fn load_ld_run_flow() -> LdRunFlowFn {
    unsafe {
        let dll_name = CString::new("loopdynamo.dll").unwrap();
        let dll = LoadLibraryA(PCSTR(dll_name.as_ptr() as _))
            .expect("failed to load loopdynamo.dll");

        let func_name = CString::new("ld_run_flow").unwrap();
        let func: FARPROC = GetProcAddress(dll, PCSTR(func_name.as_ptr() as _));

        assert!(func.is_some(), "failed to get ld_run_flow");

        std::mem::transmute(func.unwrap())
    }
}



#[tauri::command]
pub fn ld_run_flow_cmd(json: String) -> String {
    let f = load_ld_run_flow();
    let c_json = CString::new(json).unwrap();

    unsafe {
        let ptr = f(c_json.as_ptr());
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}




pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ld_run_flow_cmd])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

