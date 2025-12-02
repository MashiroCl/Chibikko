use std::{fs, sync::Mutex};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use serde::{Serialize, Deserialize};

const CONFIG_FILENAME: &str = "app_config.json";

#[derive(Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub ollm_model: String, 
    pub tts_api_endpoint: String,
    pub tts_example_wav: Option<String>,
    pub tts_example_wav_text: Option<String>
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            ollm_model:"gemma3:4b".to_string(),
            tts_api_endpoint:"http://127.0.0.1:9880".to_string(),
            tts_example_wav: None,
            tts_example_wav_text: None
        }
    }
}


pub struct ConfigManager {
    pub config: Mutex<AppConfig>,
    pub file_path: PathBuf,
}

impl ConfigManager {
    pub fn new(app_handle: &AppHandle) -> Self {
        let config_dir = app_handle.path().app_config_dir().unwrap();
        println!("Config_dir is {:?}", config_dir);

        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).expect("Failed to create config dir");
        }

        let file_path = config_dir.join(CONFIG_FILENAME);

        let config = if file_path.exists() {
            let content = fs::read_to_string(&file_path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            let def = AppConfig::default();
            let json = serde_json::to_string_pretty(&def).unwrap();
            fs::write(&file_path, json).ok();
            
            def
        };    

        Self { config: Mutex::new(config), file_path: file_path }
    }

    pub fn save(&self) -> Result<(), String> {
        let config = self.config.lock().unwrap();
        let json = serde_json::to_string_pretty(&*config).map_err(|e| e.to_string())?;

        fs::write(&self.file_path, json).map_err(|e| e.to_string())
    }

}