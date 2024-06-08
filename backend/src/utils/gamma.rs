use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{config::Config, error::HubbitResult, models::GammaUser};

#[derive(Debug, Deserialize)]
pub struct GammaTokenResponse {
  pub access_token: String,
}

#[derive(Debug, Serialize)]
struct GammaTokenRequest {
  client_id: String,
  client_secret: String,
  code: String,
  redirect_uri: String,
  grant_type: String,
}

pub async fn oauth2_token(config: &Config, code: &str) -> HubbitResult<GammaTokenResponse> {
  let client = Client::new();

  // TODO: THis should be a config param
  let redirect_uri = "http://localhost:3000/api/auth/gamma/callback";

  let url = format!("{}/oauth2/token", config.gamma_internal_url);

  let body_str = client
    .post(&url)
    .form(&GammaTokenRequest {
      client_id: config.gamma_client_id.clone(),
      client_secret: config.gamma_client_secret.clone(),
      code: code.into(),
      redirect_uri: redirect_uri.into(),
      grant_type: "authorization_code".into(),
    })
    .header("accept", "application/json")
    .send()
    .await?
    .text()
    .await?;

  Ok(serde_json::from_str(&body_str)?)
}

pub async fn get_current_user(config: &Config, access_token: &str) -> HubbitResult<GammaUser> {
  let client = Client::new();
  let url = format!("{}/oauth2/userinfo", config.gamma_internal_url);
  let body_str = client
    .get(&url)
    .bearer_auth(access_token)
    .send()
    .await?
    .text()
    .await?;

  Ok(serde_json::from_str(&body_str)?)
}
