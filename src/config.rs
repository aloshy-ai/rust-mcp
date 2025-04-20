use std::fs::{self, create_dir_all, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use dirs::config_dir;
use serde::{Deserialize, Serialize};

use crate::error::{McpError, McpResult, to_config_error};

const APP_NAME: &str = "rust-mcp";
const CONFIG_FILE: &str = "config.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    // Supabase configuration
    pub supabase_url: String,
    pub supabase_anon_key: String,
    
    // GitHub OAuth app configuration
    pub github_client_id: String,
}

impl Default for Config {
    fn default() -> Self {
        // These would normally be stored in environment variables or provided in a .env file
        // For now, we'll hardcode them for simplicity
        Self {
            supabase_url: "https://your-project.supabase.co".to_string(),
            supabase_anon_key: "your-anon-key".to_string(),
            github_client_id: "your-github-client-id".to_string(),
        }
    }
}

impl Config {
    /// Load the configuration from the config file
    pub fn load() -> McpResult<Self> {
        let config_path = get_config_path()?;
        
        if !config_path.exists() {
            // Create default config if it doesn't exist
            let config = Config::default();
            config.save()?;
            return Ok(config);
        }
        
        let mut file = File::open(&config_path).map_err(to_config_error)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(to_config_error)?;
        
        let config: Config = serde_json::from_str(&contents).map_err(to_config_error)?;
        Ok(config)
    }
    
    /// Save the configuration to the config file
    pub fn save(&self) -> McpResult<()> {
        let config_path = get_config_path()?;
        
        // Ensure the config directory exists
        if let Some(parent) = config_path.parent() {
            create_dir_all(parent).map_err(to_config_error)?;
        }
        
        let contents = serde_json::to_string_pretty(self).map_err(to_config_error)?;
        let mut file = File::create(&config_path).map_err(to_config_error)?;
        file.write_all(contents.as_bytes()).map_err(to_config_error)?;
        
        Ok(())
    }
}

/// Get the path to the config file
fn get_config_path() -> McpResult<PathBuf> {
    let config_dir = config_dir()
        .ok_or_else(|| McpError::ConfigError("Could not find config directory".to_string()))?
        .join(APP_NAME);
    
    Ok(config_dir.join(CONFIG_FILE))
}

/// Initialize environment from config or .env file
pub fn init_environment() -> McpResult<Config> {
    // Try to load from .env file if it exists
    let _ = dotenv::dotenv();
    
    // Load the config (or create default)
    let mut config = Config::load()?;
    
    // Override with environment variables if present
    if let Ok(url) = std::env::var("SUPABASE_URL") {
        config.supabase_url = url;
    }
    
    if let Ok(key) = std::env::var("SUPABASE_ANON_KEY") {
        config.supabase_anon_key = key;
    }
    
    if let Ok(client_id) = std::env::var("GITHUB_CLIENT_ID") {
        config.github_client_id = client_id;
    }
    
    Ok(config)
}
