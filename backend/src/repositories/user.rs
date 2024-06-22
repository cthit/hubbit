use gamma_rust_client::{api::GammaClient, error::GammaError};
use uuid::Uuid;

use crate::{
  config::Config,
  error::{HubbitError, HubbitResult},
  models::GammaUser,
};

#[derive(Clone)]
pub struct UserRepository {
  gamma_client: GammaClient,
}

impl UserRepository {
  pub fn new(config: Config) -> Self {
    let gamma_client = GammaClient::new(&config.gamma_config);
    Self { gamma_client }
  }

  pub async fn get(&self, id: &Uuid) -> HubbitResult<GammaUser> {
    let user = self.gamma_client.get_user(id).await.map_err(|err| {
      if matches!(err, GammaError::NotFoundResponse { .. }) {
        return HubbitError::NotFound;
      }

      HubbitError::GammaError(err)
    })?;

    let groups = self.gamma_client.get_groups_for_user(&user.id).await?;

    Ok(GammaUser::from_user_and_groups(user, groups))
  }

  pub async fn get_by_cid(&self, cid: &str) -> HubbitResult<GammaUser> {
    log::warn!(
      "Retrieving user by cid ('{cid}'), this is quite expensive and should be avoided if possible"
    );

    let user = self
      .gamma_client
      .get_users()
      .await?
      .into_iter()
      .filter(|user| user.cid.as_str() == cid)
      .next()
      .ok_or(HubbitError::NotFound)?;

    let groups = self.gamma_client.get_groups_for_user(&user.id).await?;

    Ok(GammaUser::from_user_and_groups(user, groups))
  }

  pub async fn get_all(&self) -> HubbitResult<Vec<GammaUser>> {
    let mut mapped = vec![];
    for user in self.gamma_client.get_users().await?.into_iter() {
      let groups = self.gamma_client.get_groups_for_user(&user.id).await?;
      mapped.push(GammaUser::from_user_and_groups(user, groups));
    }

    Ok(mapped)
  }
}
