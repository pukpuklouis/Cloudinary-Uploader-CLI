use anyhow::Result;
use colored::Colorize;

use crate::config::Config;
use crate::utils::{print_error, print_info};

pub fn execute() -> Result<()> {
    match Config::load() {
        Ok(config) => {
            print_info("Current Cloudinary Configuration:");
            println!("  {}: {}", "Cloud Name".cyan(), config.cloudinary.cloud_name);
            println!("  {}: {}", "API Key".cyan(), config.cloudinary.api_key);
            println!("  {}: {}", "API Secret".cyan(), mask_secret(&config.cloudinary.api_secret));
            
            if !config.cloudinary.default_folder.is_empty() {
                println!("  {}: {}", "Default Folder".cyan(), config.cloudinary.default_folder);
            } else {
                println!("  {}: {}", "Default Folder".cyan(), "(not set)".dimmed());
            }
            
            println!("\n{}: {}", "Config File".green(), Config::config_path().display());
            
            Ok(())
        }
        Err(err) => {
            print_error(&format!("Failed to load configuration: {}", err));
            print_info("Run 'cloudy init' to create a new configuration file.");
            Ok(())
        }
    }
}

fn mask_secret(secret: &str) -> String {
    if secret.len() <= 4 {
        return "*".repeat(secret.len());
    }
    
    let visible_chars = 4;
    let hidden_chars = secret.len() - visible_chars;
    
    format!("{}{}", "*".repeat(hidden_chars), &secret[hidden_chars..])
}
