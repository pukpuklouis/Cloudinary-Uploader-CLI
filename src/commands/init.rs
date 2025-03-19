use anyhow::Result;
use colored::Colorize;
use std::io::{self, Write};

use crate::config::Config;
use crate::utils::{print_info, print_success};

pub fn execute() -> Result<()> {
    print_info("Initializing Cloudinary Uploader CLI configuration...");
    
    let config_path = Config::config_path();
    if config_path.exists() {
        print_warning("Configuration file already exists at: {}", config_path.display());
        print!("Do you want to overwrite it? [y/N]: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        if !input.trim().eq_ignore_ascii_case("y") {
            print_info("Configuration initialization aborted.");
            return Ok(());
        }
    }
    
    print_info("Please enter your Cloudinary credentials:");
    
    print!("Cloud Name: ");
    io::stdout().flush()?;
    let mut cloud_name = String::new();
    io::stdin().read_line(&mut cloud_name)?;
    
    print!("API Key: ");
    io::stdout().flush()?;
    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key)?;
    
    print!("API Secret: ");
    io::stdout().flush()?;
    let mut api_secret = String::new();
    io::stdin().read_line(&mut api_secret)?;
    
    print!("Default Folder (optional): ");
    io::stdout().flush()?;
    let mut default_folder = String::new();
    io::stdin().read_line(&mut default_folder)?;
    
    let config = Config::new(
        cloud_name.trim(),
        api_key.trim(),
        api_secret.trim(),
        default_folder.trim(),
    );
    
    config.save()?;
    
    print_success(&format!(
        "Configuration file created at: {}",
        config_path.display()
    ));
    
    Ok(())
}

fn print_warning(format: &str, args: impl std::fmt::Display) {
    println!("{} {}", "⚠".yellow().bold(), format.replace("{}", &args.to_string()));
}
