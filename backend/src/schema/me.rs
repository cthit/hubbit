use async_graphql::{Context, Object};

use crate::models::AuthorizedUser;

use super::{user::User, AuthGuard};

#[derive(Default)]
pub struct MeQuery;

#[Object]
impl MeQuery {
  #[graphql(guard = AuthGuard)]
  pub async fn me(&self, context: &Context<'_>) -> User {
    let user = context.data_unchecked::<AuthorizedUser>();
    User { id: user.user_id }
  }
}
