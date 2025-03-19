use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    ReadError(#[from] std::io::Error),
    
    #[error("Failed to parse config file: {0}")]
    ParseError(#[from] toml::de::Error),
    
    #[error("Failed to write config file: {0}")]
    WriteError(String),
    
    #[error("Config file not found")]
    NotFound,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CloudinaryConfig {
    pub cloud_name: String,
    pub api_key: String,
    pub api_secret: String,
    #[serde(default)]
    pub default_folder: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub cloudinary: CloudinaryConfig,
}

impl Config {
    pub fn new(cloud_name: &str, api_key: &str, api_secret: &str, default_folder: &str) -> Self {
        Config {
            cloudinary: CloudinaryConfig {
                cloud_name: cloud_name.to_string(),
                api_key: api_key.to_string(),
                api_secret: api_secret.to_string(),
                default_folder: default_folder.to_string(),
            },
        }
    }

    pub fn default() -> Self {
        Config {
            cloudinary: CloudinaryConfig {
                cloud_name: String::new(),
                api_key: String::new(),
                api_secret: String::new(),
                default_folder: String::new(),
            },
        }
    }

    pub fn config_path() -> PathBuf {
        dirs::home_dir()
            .expect("Could not find home directory")
            .join(".cloudyrc")
    }

    pub fn load() -> Result<Self, ConfigError> {
        let config_path = Self::config_path();
        
        if !config_path.exists() {
            return Err(ConfigError::NotFound);
        }
        
        let config_str = fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&config_str)?;
        
        Ok(config)
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let config_path = Self::config_path();
        let config_str = toml::to_string_pretty(self)
            .map_err(|e| ConfigError::WriteError(e.to_string()))?;
        
        fs::write(&config_path, config_str)
            .map_err(|e| ConfigError::WriteError(e.to_string()))?;
        
        Ok(())
    }

    pub fn from_env() -> Option<Self> {
        let cloudinary_url = std::env::var("CLOUDINARY_URL").ok()?;
        
        // Format: cloudinary://<api_key>:<api_secret>@<cloud_name>
        if let Some(without_prefix) = cloudinary_url.strip_prefix("cloudinary://") {
            let parts: Vec<&str> = without_prefix.split('@').collect();
            if parts.len() == 2 {
                let credentials: Vec<&str> = parts[0].split(':').collect();
                if credentials.len() == 2 {
                    let api_key = credentials[0];
                    let api_secret = credentials[1];
                    let cloud_name = parts[1];
                    
                    return Some(Config::new(cloud_name, api_key, api_secret, ""));
                }
            }
        }
        
        None
    }
}
