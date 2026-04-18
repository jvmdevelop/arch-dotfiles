#[cfg_attr(mobile, tauri::mobile_entry_point)]
use app::command::{AppState, init_data_manager, list_objects};

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            data_manager: std::sync::Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![list_objects, init_data_manager])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
