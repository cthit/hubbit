use async_graphql::{Context, Object, SimpleObject};
use chrono::{DateTime, Utc};
use log::error;

use crate::{
  repositories::user_session::UserSessionRepository,
  schema::{user::User, AuthGuard, HubbitSchemaError, HubbitSchemaResult},
};

#[derive(Default)]
pub struct SessionQuery;

#[Object]
impl SessionQuery {
  #[graphql(guard = "AuthGuard::default()")]
  pub async fn current_sessions(
    &self,
    context: &Context<'_>,
  ) -> HubbitSchemaResult<Vec<ActiveSession>> {
    let user_session_repo = context.data_unchecked::<UserSessionRepository>();
    let active_sessions = user_session_repo.get_active().await.map_err(|e| {
      error!("[Schema error] {:?}", e);
      HubbitSchemaError::InternalError
    })?;
    Ok(
      active_sessions
        .iter()
        .map(|session| ActiveSession {
          user: User {
            id: session.user_id,
          },
          start_time: session.start_time,
        })
        .collect(),
    )
  }
}

#[derive(SimpleObject)]
pub struct ActiveSession {
  pub user: User,
  pub start_time: DateTime<Utc>,
}
