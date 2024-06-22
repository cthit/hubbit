use actix_session::Session;
use actix_web::{
  guard,
  web::{self, ServiceConfig},
  Error, HttpRequest, HttpResponse, Result,
};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use gamma_rust_client::oauth::GammaAccessToken;

use crate::{config::Config, models::AuthorizedUser, schema::HubbitSchema};

async fn playground() -> Result<HttpResponse, Error> {
  Ok(
    HttpResponse::Ok()
      .content_type("text/html; charset=utf-8")
      .body(playground_source(
        GraphQLPlaygroundConfig::new("/api/graphql").subscription_endpoint("/api/graphql"),
      )),
  )
}

async fn graphql(
  session: Session,
  gql_request: GraphQLRequest,
  schema: web::Data<HubbitSchema>,
  config: web::Data<Config>,
) -> GraphQLResponse {
  let mut request = gql_request.into_inner();
  if let Ok(Some(access_token)) = session.get::<GammaAccessToken>("gamma_access_token") {
    if let Ok(user) = access_token.get_current_user(&config.gamma_config).await {
      request = request.data(AuthorizedUser::from(user));
    }
  };

  schema.execute(request).await.into()
}

async fn graphql_ws(
  session: Session,
  config: web::Data<Config>,
  schema: web::Data<HubbitSchema>,
  req: HttpRequest,
  payload: web::Payload,
) -> Result<HttpResponse> {
  let mut authenticated = false;
  if let Ok(Some(access_token)) = session.get::<GammaAccessToken>("gamma_access_token") {
    if access_token
      .get_current_user(&config.gamma_config)
      .await
      .is_ok()
    {
      authenticated = true;
    }
  };

  if !authenticated {
    return Ok(HttpResponse::Unauthorized().finish());
  }

  GraphQLSubscription::new(HubbitSchema::clone(&*schema)).start(&req, payload)
}

pub fn init(config: &mut ServiceConfig) {
  config.service(web::resource("/graphql").guard(guard::Post()).to(graphql));
  config.service(
    web::resource("/graphql")
      .guard(guard::Get())
      .guard(guard::Header("upgrade", "websocket"))
      .to(graphql_ws),
  );
  config.service(web::resource("/graphql").guard(guard::Get()).to(playground));
}
