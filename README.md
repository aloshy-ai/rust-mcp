# Rust MCP

[![CI](https://github.com/aloshy-ai/rust-mcp/actions/workflows/ci.yml/badge.svg)](https://github.com/aloshy-ai/rust-mcp/actions/workflows/ci.yml)

A Rust CLI for Machine Control Protocol (MCP) with Supabase GitHub OAuth authentication.

## Features

- CLI commands for authentication: `signup`, `login`, and `whoami`
- Secure GitHub OAuth authentication flow using browser automation
- Integration with Supabase for user management
- Secure token storage using the system's credential manager

## Prerequisites

1. A Supabase project
2. A GitHub OAuth app
3. Rust and Cargo installed

## Setting Up Supabase

1. Create a new Supabase project at [https://app.supabase.io/](https://app.supabase.io/)
2. Go to Authentication → Providers and enable GitHub OAuth
3. Note your project URL and anon key from the API settings

## Setting Up GitHub OAuth

1. Go to your GitHub account settings → Developer settings → OAuth Apps
2. Create a new OAuth application
3. Set the Homepage URL to your project URL
4. Set the Authorization callback URL to: `https://your-project.supabase.co/auth/v1/callback`
5. Note the Client ID and Client Secret

## Configuration

Create a `.env` file in the project root (based on `.env.example`):

```
SUPABASE_URL=https://your-project.supabase.co
SUPABASE_ANON_KEY=your-supabase-anon-key
GITHUB_CLIENT_ID=your-github-client-id
```

## Getting Started

```bash
# Clone the repository
git clone https://github.com/aloshy-ai/rust-mcp.git

# Navigate to the project directory
cd rust-mcp

# Copy the environment file and fill in your values
cp .env.example .env

# Build the project
cargo build

# Sign up with a GitHub account
cargo run -- signup

# Login with your GitHub account
cargo run -- login

# Check the current logged-in user
cargo run -- whoami
```

## Browser Automation

This CLI uses browser automation with `headless_chrome` to handle the OAuth flow. It will:

1. Check if you're already logged in to GitHub
2. Open a browser window for authentication if needed
3. Automatically extract the authentication token after successful login
4. Close the browser window when done

## Security

- Authentication tokens are stored securely in your system's credential manager
- No sensitive information is stored in plain text
- OAuth tokens are handled securely through browser automation

## Development

```bash
# Run tests
cargo test

# Format code
cargo fmt

# Check for issues
cargo clippy
```

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
