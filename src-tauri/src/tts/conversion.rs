use std::{path::PathBuf};
use serde::{Serialize}; 
use reqwest::Client;
use std::error::Error;

#[derive(Debug, Serialize, Clone)]
struct Payload {
    refer_wav_path: PathBuf,
    prompt_text: String,
    prompt_language: Language,
    text: String,
    text_language: Language,
}


#[derive(Debug, Serialize, Clone)]
pub enum Language {
    #[serde(rename="zh")]
    Chinese,
    #[serde(rename="ja")]
    Japanese,
    #[serde(rename="en")]
    English,
    #[serde(rename="ko")]
    Korean,
    #[serde(rename="yue")]
    Cantonese
}



pub async fn request_sovits(s: String, url: String, wav_file: String, wav_text: String) -> Result<Vec<u8>, Box<dyn Error>> {
    let request = Payload {
        refer_wav_path: PathBuf::from(wav_file),
        prompt_text: String::from(wav_text),
        prompt_language: Language::Chinese,
        text:String::from(s),
        text_language: Language::Chinese
    }; 

    let request_json = serde_json::to_string(&request)?;


    let client = Client::new();
    let res = client.post(url)
                        .body(request_json)
                        .send()
                        .await?;
    
    if !res.status().is_success() {
        return Err("HTTP Request failed".into());
    }

    let audio_bytes = res.bytes().await?.to_vec();

    Ok(audio_bytes)

}
