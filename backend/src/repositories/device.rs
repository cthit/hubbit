use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::HubbitResult, models::Device};

#[derive(Clone, Debug)]
pub struct DeviceRepository {
  pool: PgPool,
}

impl DeviceRepository {
  pub fn new(pool: PgPool) -> Self {
    Self { pool }
  }

  pub async fn get_by_id(&self, id: Uuid) -> HubbitResult<Device> {
    Ok(
      sqlx::query_as!(
        Device,
        "
SELECT *
FROM devices
WHERE id = $1
        ",
        id
      )
      .fetch_one(&self.pool)
      .await?,
    )
  }

  pub async fn get_by_addrs(&self, mac_addrs: &[String]) -> HubbitResult<Vec<Device>> {
    Ok(
      sqlx::query_as!(
        Device,
        "
SELECT *
FROM devices
WHERE address = ANY($1)
        ",
        mac_addrs
      )
      .fetch_all(&self.pool)
      .await?,
    )
  }

  pub async fn get_for_user(&self, user_id: Uuid) -> HubbitResult<Vec<Device>> {
    Ok(
      sqlx::query_as!(
        Device,
        "
SELECT *
FROM devices
WHERE user_id = $1
        ",
        user_id
      )
      .fetch_all(&self.pool)
      .await?,
    )
  }

  pub async fn create(&self, data: CreateDevice) -> HubbitResult<Device> {
    Ok(
      sqlx::query_as!(
        Device,
        "
INSERT INTO devices (user_id, address, name)
VALUES ($1, $2, $3)
RETURNING *
        ",
        data.user_id,
        data.address,
        data.name
      )
      .fetch_one(&self.pool)
      .await?,
    )
  }

  pub async fn update(&self, addr: &str, data: UpdateDevice) -> HubbitResult<Device> {
    Ok(
      sqlx::query_as!(
        Device,
        "
UPDATE devices
SET
  address = $1,
  name = $2
WHERE address = $3
RETURNING *
        ",
        data.address,
        data.name,
        addr,
      )
      .fetch_one(&self.pool)
      .await?,
    )
  }

  pub async fn delete(&self, addr: &str) -> HubbitResult<()> {
    sqlx::query!(
      "
DELETE FROM devices
WHERE address = $1
      ",
      addr,
    )
    .execute(&self.pool)
    .await?;
    Ok(())
  }
}

pub struct CreateDevice {
  pub user_id: Uuid,
  pub address: String,
  pub name: String,
}

pub struct UpdateDevice {
  pub address: String,
  pub name: String,
}
