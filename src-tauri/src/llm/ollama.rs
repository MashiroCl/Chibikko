use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;
use reqwest::Client;
use serde::Deserialize;
use tokio::io::{AsyncBufRead, BufReader};
use ollama_rs::generation::chat::{request::ChatMessageRequest, ChatMessage};
use ollama_rs::history::ChatHistory;


pub async fn run_query(prompt: String)->Result<String, String>{
    let model = "gemma3:4b".to_string();
    let mut history:Vec<ChatMessage> = vec![];

    let ollama = Ollama::default();

    let res = ollama
                    .send_chat_messages_with_history(
                        &mut history, 
                    ChatMessageRequest::new(
                        model,
                        vec![ChatMessage::user(prompt)],
                    ),
                ).await.map_err(|e| e.to_string())?;
    
    Ok(res.message.content)

 }