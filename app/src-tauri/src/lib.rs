// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn run_flow(json: String) -> String {
    unsafe {
        let cstr = std::ffi::CString::new(json).unwrap();
        let result = ld_run_flow(cstr.as_ptr());
        std::ffi::CStr::from_ptr(result).to_string_lossy().into_owned()
    }
}

extern "C" {
    fn ld_run_flow(json: *const std::os::raw::c_char) -> *const std::os::raw::c_char;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            run_flow
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
