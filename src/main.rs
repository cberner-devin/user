use std::fs;
use serde::{Deserialize, Serialize};
use reqwest::blocking::Client;

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home = dirs::home_dir().ok_or("Failed to get home directory")?;
    let readme_path = home.join("README.md");
    let contents = fs::read_to_string(&readme_path)?;
    
    println!("README.md length: {} bytes", contents.len());
    
    let api_key = std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| "sk-dummy-key".to_string());
    
    let messages = vec![
        Message {
            role: "user".to_string(),
            content: format!("Please summarize the following README content:\n\n{}", contents),
        },
    ];
    
    let request = ChatRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages,
        max_tokens: 512,
        temperature: 0.7,
    };
    
    println!("Sending README content to LLM...");
    
    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()?;
    
    if response.status().is_success() {
        let chat_response: ChatResponse = response.json()?;
        if let Some(choice) = chat_response.choices.first() {
            println!("\nLLM Response:");
            println!("{}", choice.message.content);
        } else {
            println!("No response from LLM");
        }
    } else {
        println!("Error from OpenAI API: {}", response.status());
        println!("Response: {}", response.text()?);
    }
    
    Ok(())
}
