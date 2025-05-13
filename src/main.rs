use std::fs;
use std::path::PathBuf;
use std::env;

fn main() {
    let home_dir = env::var("HOME").expect("Failed to get home directory");
    let readme_path = PathBuf::from(home_dir).join("README.md");
    
    match fs::read_to_string(&readme_path) {
        Ok(contents) => {
            println!("README.md length: {} bytes", contents.len());
        },
        Err(e) => {
            eprintln!("Error reading README.md: {}", e);
        }
    }
}
