use std::fs;
use llm::{builder::{LLMBackend, LLMBuilder}, chat::ChatMessage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home = dirs::home_dir().ok_or("Failed to get home directory")?;
    let readme_path = home.join("README.md");
    let contents = fs::read_to_string(&readme_path)?;
    
    println!("README.md length: {} bytes", contents.len());
    
    let api_key = std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| "sk-dummy-key".to_string());
    
    let llm = LLMBuilder::new()
        .backend(LLMBackend::OpenAI)
        .api_key(&api_key)
        .model("gpt-3.5-turbo")
        .max_tokens(512)
        .temperature(0.7)
        .stream(false)
        .build()?;
    
    let prompt = format!("Please summarize the following README content:\n\n{}", contents);
    
    let messages = vec![
        ChatMessage::user().content(&prompt).build(),
    ];
    
    println!("Sending README content to LLM...");
    
    match llm.chat(&messages).await {
        Ok(text) => {
            println!("\nLLM Response:");
            println!("{}", text);
        },
        Err(e) => eprintln!("Chat error: {}", e),
    }
    
    Ok(())
}
