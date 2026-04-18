use sb_core::core::manager::data_manager::DataManager;
use sb_core::core::config::Config;
use tauri::State;
use std::sync::Mutex;

pub struct AppState {
    pub data_manager: Mutex<Option<DataManager>>,
}

#[tauri::command]
pub async fn list_objects(state: State<'_, AppState>, prefix: String) -> Result<Vec<String>, String> {
    let data_manager_guard = state.data_manager.lock().unwrap();
    
    match data_manager_guard.as_ref() {
        Some(data_manager) => {
            match data_manager.list_objects("default-bucket", &prefix).await {
                Ok(objects) => Ok(objects),
                Err(e) => Err(format!("Failed to list objects: {}", e)),
            }
        }
        None => Err("Data manager not initialized".to_string()),
    }
}

#[tauri::command]
pub async fn init_data_manager(config: Config, state: State<'_, AppState>) -> Result<(), String> {
    match config.create_s3_client().await {
        Ok(s3_client) => {
            let data_manager = DataManager::new(s3_client);
            let mut guard = state.data_manager.lock().unwrap();
            *guard = Some(data_manager);
            Ok(())
        }
        Err(e) => Err(format!("Failed to create S3 client: {}", e)),
    }
}
