use std::os::raw::c_char;

#[link(name = "loopdynamo", kind = "dylib")]
extern "C" {
    fn ld_run_flow(json: *const c_char) -> *const c_char;
}

pub fn run_flow(json: &str) -> String {
    use std::ffi::{CString, CStr};

    let c_json = CString::new(json).unwrap();

    unsafe {
        let ptr = ld_run_flow(c_json.as_ptr());
        let c_str = CStr::from_ptr(ptr);
        c_str.to_string_lossy().into_owned()
    }
}

#[tauri::command]
fn run_flow_cmd(json: String) -> String {
    run_flow(&json)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            run_flow_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
