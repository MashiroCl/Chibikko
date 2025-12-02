pub mod ollama;



pub async fn run_ollama_query(prompt:String, model: String)->Result<String, String>{
    ollama::run_query(prompt, model).await

}