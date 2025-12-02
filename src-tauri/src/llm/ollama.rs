use ollama_rs::Ollama;
use ollama_rs::generation::chat::{request::ChatMessageRequest, ChatMessage};

pub async fn run_query(prompt: String, model: String)->Result<String, String>{
    let mut history:Vec<ChatMessage> = vec![];

    let ollama = Ollama::default();
    println!("Querying the LLM {}", prompt);

    let res = ollama
                    .send_chat_messages_with_history(
                        &mut history, 
                    ChatMessageRequest::new(
                        model,
                        vec![ChatMessage::user(prompt)],
                    ),
                ).await.map_err(|e| e.to_string())?;
    
    println!("Response is {}", res.message.content);
    Ok(res.message.content)

 }