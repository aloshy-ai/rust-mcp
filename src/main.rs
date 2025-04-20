mod auth;
mod browser;
mod config;
mod error;
mod supabase;

use clap::{Parser, Subcommand};
use std::process;

use crate::auth::AuthHandler;
use crate::config::init_environment;
use crate::error::McpResult;

#[derive(Parser)]
#[clap(
    name = "rust-mcp",
    about = "Rust Machine Control Protocol CLI",
    version
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Sign up with a new GitHub account
    Signup,
    
    /// Login with an existing GitHub account
    Login,
    
    /// Show the current logged-in user
    Whoami,
}

#[tokio::main]
async fn main() -> McpResult<()> {
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Initialize environment and load config
    let config = match init_environment() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error initializing environment: {}", err);
            process::exit(1);
        }
    };
    
    // Create auth handler
    let auth_handler = AuthHandler::new(config);
    
    // Process the command
    match cli.command {
        Commands::Signup => {
            if let Err(err) = auth_handler.signup().await {
                eprintln!("Signup failed: {}", err);
                process::exit(1);
            }
        }
        Commands::Login => {
            if let Err(err) = auth_handler.login().await {
                eprintln!("Login failed: {}", err);
                process::exit(1);
            }
        }
        Commands::Whoami => {
            if let Err(err) = auth_handler.whoami().await {
                eprintln!("Error: {}", err);
                process::exit(1);
            }
        }
    }
    
    Ok(())
}
