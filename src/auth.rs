use keyring::Keyring;
use std::fmt;

use crate::browser::BrowserAutomation;
use crate::config::Config;
use crate::error::{McpError, McpResult, to_credential_error};
use crate::supabase::{SupabaseClient, UserProfile};

const SERVICE_NAME: &str = "rust-mcp";
const USERNAME: &str = "supabase-token";

/// Authentication handler for the CLI
pub struct AuthHandler {
    config: Config,
    supabase: SupabaseClient,
}

impl AuthHandler {
    /// Create a new authentication handler
    pub fn new(config: Config) -> Self {
        let supabase = SupabaseClient::new(config.clone());
        Self { config, supabase }
    }
    
    /// Sign up a new user through GitHub OAuth
    pub async fn signup(&self) -> McpResult<()> {
        println!("Initiating signup process with GitHub...");
        
        // Perform the OAuth flow
        let token = self.perform_oauth_flow(true).await?;
        
        // Store the token
        self.store_token(&token)?;
        
        // Get user profile to confirm it worked
        let user = self.supabase.get_user_profile(&token).await?;
        
        println!("Signup successful!");
        self.print_user_info(&user);
        
        Ok(())
    }
    
    /// Log in an existing user through GitHub OAuth
    pub async fn login(&self) -> McpResult<()> {
        println!("Initiating login process with GitHub...");
        
        // Perform the OAuth flow
        let token = self.perform_oauth_flow(false).await?;
        
        // Store the token
        self.store_token(&token)?;
        
        // Get user profile to confirm it worked
        let user = self.supabase.get_user_profile(&token).await?;
        
        println!("Login successful!");
        self.print_user_info(&user);
        
        Ok(())
    }
    
    /// Get the current user profile using the stored token
    pub async fn whoami(&self) -> McpResult<()> {
        // Get the stored token
        let token = self.get_token()?;
        
        // Get the user profile
        let user = self.supabase.get_user_profile(&token).await?;
        
        println!("Currently logged in as:");
        self.print_user_info(&user);
        
        Ok(())
    }
    
    /// Perform the OAuth flow and return the token
    async fn perform_oauth_flow(&self, is_signup: bool) -> McpResult<String> {
        // Create browser automation
        let browser = BrowserAutomation::new()?;
        
        // Check if already logged in to GitHub
        let is_logged_in = browser.is_github_logged_in().await?;
        if is_logged_in {
            println!("User is already logged in to GitHub. Using existing session.");
        } else {
            println!("GitHub login required. Please log in using the browser window.");
        }
        
        // Build the auth URL
        let auth_url = self.supabase.build_github_auth_url(is_signup)?;
        
        // Get the callback URL prefix for checking success
        let callback_url_prefix = self.supabase.get_callback_url_prefix();
        
        // Open browser and wait for authentication
        println!("Opening browser for authentication...");
        let final_url = browser.authenticate_with_github(&auth_url, &callback_url_prefix).await?;
        
        // Extract the token from the URL
        let token = browser.extract_token_from_url(&final_url)?;
        
        Ok(token)
    }
    
    /// Store the authentication token securely
    fn store_token(&self, token: &str) -> McpResult<()> {
        let keyring = Keyring::new(SERVICE_NAME, USERNAME);
        keyring.set_password(token).map_err(to_credential_error)?;
        
        Ok(())
    }
    
    /// Get the stored authentication token
    fn get_token(&self) -> McpResult<String> {
        let keyring = Keyring::new(SERVICE_NAME, USERNAME);
        let token = keyring.get_password().map_err(|_| McpError::NotAuthenticated)?;
        
        Ok(token)
    }
    
    /// Print user information in a formatted way
    fn print_user_info(&self, user: &UserProfile) {
        println!("User ID: {}", user.id);
        
        if let Some(email) = &user.email {
            println!("Email: {}", email);
        }
        
        if let Some(name) = &user.user_metadata.name {
            println!("Name: {}", name);
        } else if let Some(full_name) = &user.user_metadata.full_name {
            println!("Name: {}", full_name);
        }
        
        if let Some(username) = &user.user_metadata.preferred_username {
            println!("GitHub Username: {}", username);
        } else if let Some(username) = &user.user_metadata.user_name {
            println!("GitHub Username: {}", username);
        }
        
        println!("Provider: {}", user.app_metadata.provider);
        println!("Account created at: {}", user.created_at);
    }
}

impl fmt::Display for UserProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "User(id={}", self.id)?;
        
        if let Some(email) = &self.email {
            write!(f, ", email={}", email)?;
        }
        
        if let Some(name) = &self.user_metadata.name {
            write!(f, ", name={}", name)?;
        } else if let Some(full_name) = &self.user_metadata.full_name {
            write!(f, ", name={}", full_name)?;
        }
        
        if let Some(username) = &self.user_metadata.preferred_username {
            write!(f, ", username={}", username)?;
        } else if let Some(username) = &self.user_metadata.user_name {
            write!(f, ", username={}", username)?;
        }
        
        write!(f, ")")
    }
}
