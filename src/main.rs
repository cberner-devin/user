use std::fs;
use llm::{builder::{LLMBackend, LLMBuilder}, chat::ChatMessage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home = dirs::home_dir().ok_or("Failed to get home directory")?;
    let readme_path = home.join("README.md");
    let contents = fs::read_to_string(&readme_path)?;
    
    println!("README.md length: {} bytes", contents.len());
    
    let api_key = std::env::var("OPENAI_API_KEY")
        .map_err(|_| "OPENAI_API_KEY environment variable not set")?;
    
    let llm = LLMBuilder::new()
        .backend(LLMBackend::OpenAI)
        .api_key(&api_key)
        .model("gpt-3.5-turbo")
        .max_tokens(512)
        .stream(false)
        .build()?;
    
    let messages = vec![
        ChatMessage::user().content(&contents).build(),
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
