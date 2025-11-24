// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod state;
mod keylogger;

use crate::state::AppState;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Chibikko!", name)
}

#[tauri::command]
fn get_status(state: tauri::State<AppState>) -> String{
    let data = state.key_counter.lock().unwrap();
    serde_json::to_string(&*data).unwrap()
    
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    let app_state = AppState::new();

    keylogger::start(&app_state);

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
