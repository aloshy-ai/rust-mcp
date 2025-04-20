use thiserror::Error;

#[derive(Debug, Error)]
pub enum McpError {
    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Browser automation error: {0}")]
    BrowserError(String),

    #[error("HTTP request error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Credential storage error: {0}")]
    CredentialError(String),

    #[error("Config error: {0}")]
    ConfigError(String),

    #[error("JSON serialization error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Not authenticated. Please login first.")]
    NotAuthenticated,

    #[error("Unexpected error: {0}")]
    Other(String),
}

pub type McpResult<T> = Result<T, McpError>;

// Helper functions for common error conversions
pub(crate) fn to_auth_error<E: std::fmt::Display>(e: E) -> McpError {
    McpError::AuthError(e.to_string())
}

pub(crate) fn to_browser_error<E: std::fmt::Display>(e: E) -> McpError {
    McpError::BrowserError(e.to_string())
}

pub(crate) fn to_credential_error<E: std::fmt::Display>(e: E) -> McpError {
    McpError::CredentialError(e.to_string())
}

pub(crate) fn to_config_error<E: std::fmt::Display>(e: E) -> McpError {
    McpError::ConfigError(e.to_string())
}

pub(crate) fn to_other_error<E: std::fmt::Display>(e: E) -> McpError {
    McpError::Other(e.to_string())
}
