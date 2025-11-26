pub mod ollama;


// pub async fn run_ollama_query(prompt:String)->Result<String, String>{
//     ollama::run_query(prompt).await

// }

pub async fn run_ollama_query(prompt:String)->Result<String, String>{
    ollama::run_query(prompt).await

}