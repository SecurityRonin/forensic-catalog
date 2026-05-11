//! Shared GitHub HTTP client with optional token auth.
//!
//! Reads `GITHUB_TOKEN` or `GH_TOKEN` from the environment and adds an
//! `Authorization: Bearer` header if found. Without a token, GitHub's
//! unauthenticated rate limit is 60 req/hour; with one it's 5000/hour.

use reqwest::blocking::{Client, ClientBuilder};

/// Build a reqwest blocking client configured for GitHub API access.
///
/// Picks up `GITHUB_TOKEN` or `GH_TOKEN` from the environment for auth.
/// Falls back to unauthenticated if neither is set.
pub fn github_client() -> Result<Client, reqwest::Error> {
    let token = std::env::var("GITHUB_TOKEN")
        .or_else(|_| std::env::var("GH_TOKEN"))
        .ok();

    let mut builder = ClientBuilder::new()
        .user_agent("forensicnomicon-ingest/0.1")
        .timeout(std::time::Duration::from_secs(30));

    if let Some(tok) = token {
        let mut headers = reqwest::header::HeaderMap::new();
        if let Ok(val) = reqwest::header::HeaderValue::from_str(&format!("Bearer {tok}")) {
            headers.insert(reqwest::header::AUTHORIZATION, val);
        }
        builder = builder.default_headers(headers);
    }

    builder.build()
}
