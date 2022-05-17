use actix_session::Session;
use actix_web::{
  web::{self, ServiceConfig},
  HttpResponse,
};
use log::{error, warn};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
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
  if let Ok(Some(access_token)) = session.get::<String>("gamma_access_token") {
    if crate::utils::gamma::get_current_user(&config, &access_token)
      .await
      .is_ok()
    {
      let url = query.from.clone().unwrap_or_else(|| "/".to_string());
      return HttpResponse::TemporaryRedirect()
        .append_header(("Location", url))
        .finish();
    }
  };

  let state: String = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(32)
    .map(char::from)
    .collect();
  match session.insert("gamma_state", &state) {
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

  let url = format!(
    "{}/api/oauth/authorize?response_type=code&client_id={}&state={}",
    config.gamma_public_url, config.gamma_client_id, state
  );
  HttpResponse::TemporaryRedirect()
    .append_header(("Location", url))
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
  let saved_state = match session.get::<String>("gamma_state") {
    Ok(Some(saved_state)) => saved_state,
    _ => {
      warn!("[Gamma auth] Could not retrieve gamma_state");
      return HttpResponse::InternalServerError().finish();
    }
  };

  if query.state != saved_state {
    warn!("[Gamma auth] State mismatch");
    return HttpResponse::BadRequest().finish();
  }

  let token_response = match crate::utils::gamma::oauth2_token(&config, &query.code).await {
    Ok(token_response) => token_response,
    Err(_) => {
      error!("[Gamma auth] Could not get gamma access token");
      return HttpResponse::BadRequest().finish();
    }
  };

  match session.insert("gamma_access_token", token_response.access_token) {
    Ok(_) => {}
    Err(_) => {
      error!("[Gamma auth] Could not set gamma_acess_token key in cookie");
      return HttpResponse::InternalServerError().finish();
    }
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
