use std::{env, str::FromStr};

#[derive(Clone, Debug)]
pub struct Config {
  pub port: u16,
  pub db_url: String,
  pub redis_url: String,
  pub gamma_public_url: String,
  pub gamma_internal_url: String,
  pub gamma_api_key: String,
  pub gamma_client_id: String,
  pub gamma_client_secret: String,
  pub gamma_redirect_uri: String,
  pub cookie_secret: String,
  pub cookie_secure: bool,
  pub group_whitelist: Vec<String>,
}

impl Config {
  pub fn from_env() -> Result<Self, ConfigError> {
    let conf = Self {
      port: try_read_var("PORT")?,
      db_url: try_read_var("DATABASE_URL")?,
      redis_url: try_read_var("REDIS_URL")?,
      gamma_public_url: try_read_var("GAMMA_PUBLIC_URL")?,
      gamma_internal_url: try_read_var("GAMMA_INTERNAL_URL")?,
      gamma_api_key: try_read_var("GAMMA_API_KEY")?,
      gamma_client_id: try_read_var("GAMMA_CLIENT_ID")?,
      gamma_client_secret: try_read_var("GAMMA_CLIENT_SECRET")?,
      gamma_redirect_uri: try_read_var("GAMMA_REIDRECT_URI")?,
      cookie_secret: try_read_var("COOKIE_SECRET")?,
      cookie_secure: try_read_var("COOKIE_SECURE")?,
      group_whitelist: try_read_var::<String>("GROUP_WHITELIST")
        .unwrap_or_else(|_| String::new())
        .split(',')
        .map(|str| str.trim().to_string())
        .filter(|str| !str.is_empty())
        .collect(),
    };

    if conf.group_whitelist.is_empty() {
      println!("No group whitelist provided, showing all");
    }

    Ok(conf)
  }
}

fn try_read_var<T: FromStr>(name: &str) -> Result<T, ConfigError> {
  let value = env::var(name).map_err(|_| ConfigError::UndefinedVar(name.to_string()))?;
  value
    .parse::<T>()
    .map_err(|_| ConfigError::InvalidVar(name.to_string()))
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum ConfigError {
  #[error("Environment variable {0} not defined")]
  UndefinedVar(String),
  #[error("Environment variable {0} is invalid")]
  InvalidVar(String),
}
