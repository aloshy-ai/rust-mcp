use headless_chrome::{Browser, LaunchOptions};
use std::sync::Arc;
use std::time::Duration;
use url::Url;

use crate::error::{McpError, McpResult, to_browser_error};

/// Timeout for browser operations in seconds
const BROWSER_TIMEOUT: u64 = 60;

/// Handles browser automation for the OAuth flow
pub struct BrowserAutomation {
    browser: Arc<Browser>,
}

impl BrowserAutomation {
    /// Create a new browser automation instance
    pub fn new() -> McpResult<Self> {
        let options = LaunchOptions {
            headless: false, // Set to true for production, false for debugging
            ..Default::default()
        };
        
        let browser = Browser::new(options).map_err(to_browser_error)?;
        
        Ok(Self {
            browser: Arc::new(browser),
        })
    }
    
    /// Open a URL in the browser and wait for a redirect to the success URL
    /// Returns the final URL which should contain the authentication token
    pub async fn authenticate_with_github(&self, auth_url: &str, success_url_prefix: &str) -> McpResult<String> {
        // Create a new tab
        let tab = self.browser
            .new_tab()
            .map_err(to_browser_error)?;
        
        // Navigate to the authentication URL
        tab.navigate_to(auth_url)
            .map_err(to_browser_error)?;
        
        // Wait for the page to load
        tab.wait_until_navigated()
            .map_err(to_browser_error)?;
        
        let start_time = std::time::Instant::now();
        
        // Poll the current URL until we get to the success URL or timeout
        while start_time.elapsed().as_secs() < BROWSER_TIMEOUT {
            // Get the current URL
            let current_url = tab.get_url();
            
            // Check if we've reached the success URL
            if current_url.starts_with(success_url_prefix) {
                // Close the tab
                tab.close(true).map_err(to_browser_error)?;
                
                return Ok(current_url);
            }
            
            // Sleep for a short time before checking again
            std::thread::sleep(Duration::from_millis(500));
        }
        
        // If we got here, we timed out
        Err(McpError::BrowserError("Authentication timed out".to_string()))
    }
    
    /// Extract the access token from a URL or page content
    pub fn extract_token_from_url(&self, url: &str) -> McpResult<String> {
        // Parse the URL
        let parsed_url = Url::parse(url).map_err(to_browser_error)?;
        
        // First try to get the token from the hash fragment
        if let Some(fragment) = parsed_url.fragment() {
            // The fragment might be something like "access_token=xyz&token_type=bearer"
            let params: Vec<&str> = fragment.split('&').collect();
            
            for param in params {
                let kv: Vec<&str> = param.split('=').collect();
                if kv.len() == 2 && kv[0] == "access_token" {
                    return Ok(kv[1].to_string());
                }
            }
        }
        
        // If not found in fragment, try the query parameters
        for (key, value) in parsed_url.query_pairs() {
            if key == "access_token" {
                return Ok(value.to_string());
            }
        }
        
        // If we couldn't find the token
        Err(McpError::AuthError("Could not extract access token from URL".to_string()))
    }
    
    /// Check if user is already logged in to GitHub by opening a test page
    pub async fn is_github_logged_in(&self) -> McpResult<bool> {
        // Create a new tab
        let tab = self.browser
            .new_tab()
            .map_err(to_browser_error)?;
        
        // Navigate to GitHub
        tab.navigate_to("https://github.com")
            .map_err(to_browser_error)?;
        
        // Wait for the page to load
        tab.wait_until_navigated()
            .map_err(to_browser_error)?;
        
        // Check for elements that would indicate a logged-in state
        // This is a simple heuristic and might need adjustment
        let logged_in = tab.find_element("summary.Header-link[aria-label='View profile and more']")
            .is_ok();
        
        // Close the tab
        tab.close(true).map_err(to_browser_error)?;
        
        Ok(logged_in)
    }
}
