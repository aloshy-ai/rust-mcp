use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use url::Url;

use crate::config::Config;
use crate::error::{McpError, McpResult, to_auth_error};

/// Supabase client for interacting with Supabase APIs
pub struct SupabaseClient {
    client: Client,
    config: Config,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    id: String,
    email: Option<String>,
    user_metadata: UserMetadata,
    app_metadata: AppMetadata,
    created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserMetadata {
    avatar_url: Option<String>,
    email: Option<String>,
    email_verified: Option<bool>,
    full_name: Option<String>,
    iss: Option<String>,
    name: Option<String>,
    preferred_username: Option<String>,
    provider_id: Option<String>,
    sub: Option<String>,
    user_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppMetadata {
    provider: String,
    providers: Vec<String>,
}

impl SupabaseClient {
    /// Create a new Supabase client
    pub fn new(config: Config) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");
        
        Self { client, config }
    }
    
    /// Build the GitHub OAuth URL for signup or login
    pub fn build_github_auth_url(&self, is_signup: bool) -> McpResult<String> {
        let base_url = format!("{}/auth/v1/authorize", self.config.supabase_url);
        
        let mut url = Url::parse(&base_url).map_err(to_auth_error)?;
        
        // Add query parameters
        url.query_pairs_mut()
            .append_pair("provider", "github")
            .append_pair("client_id", &self.config.github_client_id)
            .append_pair("redirect_to", &format!("{}/auth/v1/callback", self.config.supabase_url))
            .append_pair("response_type", "token")
            .append_pair("scopes", "user:email")
            .append_pair("state", &generate_state());
        
        if is_signup {
            url.query_pairs_mut().append_pair("flow_type", "signup");
        }
        
        Ok(url.to_string())
    }
    
    /// Get the callback URL prefix for checking successful authentication
    pub fn get_callback_url_prefix(&self) -> String {
        format!("{}/auth/v1/callback", self.config.supabase_url)
    }
    
    /// Get the current user profile using the provided token
    pub async fn get_user_profile(&self, token: &str) -> McpResult<UserProfile> {
        let url = format!("{}/auth/v1/user", self.config.supabase_url);
        
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(&format!("Bearer {}", token))
                .map_err(to_auth_error)?,
        );
        headers.insert(
            "apikey",
            header::HeaderValue::from_str(&self.config.supabase_anon_key)
                .map_err(to_auth_error)?,
        );
        
        let response = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await
            .map_err(McpError::HttpError)?;
        
        if !response.status().is_success() {
            return Err(McpError::AuthError(format!(
                "Failed to get user profile: HTTP {}", 
                response.status()
            )));
        }
        
        let user_profile = response
            .json::<UserProfile>()
            .await
            .map_err(McpError::HttpError)?;
        
        Ok(user_profile)
    }
}

/// Generate a random state string for the OAuth flow
fn generate_state() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    
    let random = rand::random::<u64>();
    
    format!("{}{}", timestamp, random)
}
