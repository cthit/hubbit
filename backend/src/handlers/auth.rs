use actix_session::Session;
use actix_web::{
  web::{self, ServiceConfig},
  HttpResponse,
};
use gamma_rust_client::oauth::{GammaAccessToken, GammaState};
use log::{error, warn};
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct GammaInitFlowQuery {
  from: Option<String>,
}

async fn gamma_init_flow(
  config: web::Data<Config>,
  session: Session,
  query: web::Query<GammaInitFlowQuery>,
) -> HttpResponse {
  if let Ok(Some(access_token)) = session.get::<GammaAccessToken>("gamma_access_token") {
    if let Err(err) = access_token.get_current_user(&config.gamma_config).await {
      warn!("[Gamma auth flow] Failed to get current user with the access token, err: {err:?}");
      session.remove("gamma_access_token");
    } else {
      // The user is already authenticated.
      let url = query.from.clone().unwrap_or_else(|| "/".to_string());
      return HttpResponse::TemporaryRedirect()
        .append_header(("Location", url))
        .finish();
    }
  };

  let gamma_init = match gamma_rust_client::oauth::gamma_init_auth(&config.gamma_config) {
    Ok(init) => init,
    Err(err) => {
      error!("[Gamma auth] Could not setup gamma auth initialization, err: {err:?}");
      return HttpResponse::InternalServerError().finish();
    }
  };

  match session.insert("gamma_state", gamma_init.state) {
    Ok(_) => {}
    Err(_) => {
      error!("[Gamma auth] Could not set gamma_state key in cookie");
      return HttpResponse::InternalServerError().finish();
    }
  }

  match &query.from {
    Some(from) => match session.insert("gamma_from", from) {
      Ok(_) => {}
      Err(_) => {
        error!("[Gamma auth] Could not set gamma_from key in cookie");
        return HttpResponse::InternalServerError().finish();
      }
    },
    None => {
      session.remove("gamma_from");
    }
  }

  HttpResponse::TemporaryRedirect()
    .append_header(("Location", gamma_init.redirect_to))
    .finish()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GammaTokenQuery {
  code: String,
  state: String,
}

async fn gamma_callback(
  config: web::Data<Config>,
  session: Session,
  query: web::Query<GammaTokenQuery>,
) -> HttpResponse {
  let saved_state = match session.get::<GammaState>("gamma_state") {
    Ok(Some(saved_state)) => saved_state,
    _ => {
      warn!("[Gamma auth] Could not retrieve gamma_state");
      return HttpResponse::InternalServerError().finish();
    }
  };

  let token = match saved_state
    .gamma_callback_params(
      &config.gamma_config,
      query.state.clone(),
      query.code.clone(),
    )
    .await
  {
    Ok(t) => t,
    Err(err) => {
      error!("[Gamma auth] Failed to exchange code for auth token, err: {err:?}");
      return HttpResponse::BadRequest().finish();
    }
  };

  if let Err(_) = session.insert("gamma_access_token", token) {
    error!("[Gamma auth] Could not set gamma_acess_token key in cookie");
    return HttpResponse::InternalServerError().finish();
  }

  let from = match session.get::<String>("gamma_from") {
    Ok(Some(from)) => from,
    _ => "/".to_string(),
  };

  session.remove("gamma_from");
  session.remove("gamma_state");

  HttpResponse::TemporaryRedirect()
    .append_header(("Location", from))
    .finish()
}

pub fn init(config: &mut ServiceConfig) {
  config
    .service(web::resource("/auth/gamma/login").route(web::get().to(gamma_init_flow)))
    .service(web::resource("/auth/gamma/callback").route(web::get().to(gamma_callback)));
}
