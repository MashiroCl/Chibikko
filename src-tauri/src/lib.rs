// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod state;
mod keylogger;
mod llm;
mod tts;
mod config;

use serde::Serialize;
use tauri::{Manager, State};

use crate::{config::ConfigManager, state::AppState};


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Chibikko!", name)
}

#[tauri::command]
fn get_status(state: tauri::State<AppState>) -> String{
    let data = state.key_counter.lock().unwrap();
    serde_json::to_string(&*data).unwrap()
}

#[derive(Serialize)]
pub struct LlmResponse{
    pub text: String,
    pub audio: Option<Vec<u8>>,
}

#[tauri::command]
async fn query_llm(prompt:String, need_voice: bool, state: State<'_, ConfigManager>)->Result<LlmResponse, String>{
    let (model_name, tts_endpoint, wav_file, wav_text) = {
        let config = state.config.lock().unwrap();
        (
            config.ollm_model.clone(),
            config.tts_api_endpoint.clone(),
            config.tts_example_wav.clone(),
            config.tts_example_wav_text.clone()
        )
    };
    let res_text = llm::run_ollama_query(prompt, model_name).await.map_err(|e| e.to_string())?;
    let mut audio_data = None;
    if need_voice {
        let file = wav_file.ok_or("Example wav file not set, consider set in the directory and write in Config")?;
        let text = wav_text.ok_or("Example wav text not set, consider write in Config")?;
        let wav_bytes = tts::conversion::request_sovits(res_text.clone(),
                                                                tts_endpoint,
                                                                file,
                                                                text)
                                                                .await
                                                                .map_err(|e| e.to_string())?;
        audio_data = Some(wav_bytes);
    }

    Ok(LlmResponse { text: res_text, audio: audio_data })
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    let app_state = AppState::new();

    keylogger::start(&app_state);

    tauri::Builder::default()
    .setup(|app| {
        let config_manager = ConfigManager::new(app.handle());
        app.manage(config_manager);
        Ok(())
    })
        .manage(app_state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_status, query_llm])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
