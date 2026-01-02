use serde::{Deserialize, Serialize};
use reqwest::Client;
use crate::config::Config;
use anyhow::{Result, Context};

#[derive(Debug, Serialize, Deserialize)]
pub struct OidcConfig {
    pub issuer: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

#[derive(Debug, Deserialize)]
pub struct OidcDiscovery {
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: String,
    pub issuer: String,
}

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub id_token: Option<String>,
    pub expires_in: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub sub: String,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
}

pub async fn discover_oidc_config(issuer: &str) -> Result<OidcDiscovery> {
    let discovery_url = format!("{}/.well-known/openid-configuration", issuer.trim_end_matches('/'));
    let client = Client::new();
    let response = client
        .get(&discovery_url)
        .send()
        .await
        .context("Failed to fetch OIDC discovery document")?;
    
    response
        .json::<OidcDiscovery>()
        .await
        .context("Failed to parse OIDC discovery document")
}

pub async fn exchange_code_for_token(
    code: &str,
    config: &Config,
    discovery: &OidcDiscovery,
) -> Result<TokenResponse> {
    let client = Client::new();
    let client_id = config.oidc_client_id.as_ref()
        .ok_or_else(|| anyhow::anyhow!("OIDC client ID not configured"))?;
    let client_secret = config.oidc_client_secret.as_ref()
        .ok_or_else(|| anyhow::anyhow!("OIDC client secret not configured"))?;
    let redirect_uri = config.oidc_redirect_uri.as_ref()
        .ok_or_else(|| anyhow::anyhow!("OIDC redirect URI not configured"))?;

    let params = [
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", redirect_uri),
        ("client_id", client_id),
        ("client_secret", client_secret),
    ];

    let response = client
        .post(&discovery.token_endpoint)
        .form(&params)
        .send()
        .await
        .context("Failed to exchange authorization code for token")?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(anyhow::anyhow!("Token exchange failed: {}", error_text));
    }

    response
        .json::<TokenResponse>()
        .await
        .context("Failed to parse token response")
}

pub async fn get_user_info(access_token: &str, discovery: &OidcDiscovery) -> Result<UserInfo> {
    let client = Client::new();
    let response = client
        .get(&discovery.userinfo_endpoint)
        .bearer_auth(access_token)
        .send()
        .await
        .context("Failed to fetch user info")?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(anyhow::anyhow!("Failed to get user info: {}", error_text));
    }

    response
        .json::<UserInfo>()
        .await
        .context("Failed to parse user info")
}

