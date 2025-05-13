use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home = dirs::home_dir().ok_or("Failed to get home directory")?;
    let readme_path = home.join("README.md");
    
    let contents = fs::read_to_string(&readme_path)?;
    println!("README.md length: {} bytes", contents.len());
    
    Ok(())
}
