use async_graphql::Enum;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct Device {
  pub id: Uuid,
  pub user_id: Uuid,
  pub address: String,
  pub name: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Session {
  pub id: Uuid,
  pub user_id: Uuid,
  pub mac_address: String,
  pub start_time: DateTime<Utc>,
  pub end_time: DateTime<Utc>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct UserSession {
  pub id: Uuid,
  pub user_id: Uuid,
  pub start_time: DateTime<Utc>,
  pub end_time: DateTime<Utc>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ApiKey {
  pub id: Uuid,
  pub token: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct StudyYear {
  pub id: Uuid,
  pub year: i32,
  pub start_date: NaiveDate,
  pub end_date: NaiveDate,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct StudyPeriod {
  pub id: Uuid,
  pub year: i32,
  pub period: i32,
  pub start_date: NaiveDate,
  pub end_date: NaiveDate,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Enum, Eq, PartialEq)]
pub enum Period {
  Summer,
  LP1,
  LP2,
  LP3,
  LP4,
}

impl From<i32> for Period {
  fn from(value: i32) -> Self {
    match value {
      0 => Self::Summer,
      1 => Self::LP1,
      2 => Self::LP2,
      3 => Self::LP3,
      4 => Self::LP4,
      _ => panic!("Period integer value must be between 0 and 4"),
    }
  }
}

impl From<Period> for i32 {
  fn from(period: Period) -> Self {
    match period {
      Period::Summer => 0,
      Period::LP1 => 1,
      Period::LP2 => 2,
      Period::LP3 => 3,
      Period::LP4 => 4,
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GammaUser {
  pub id: Uuid,
  pub cid: String,
  pub nick: String,
  pub first_name: String,
  pub last_name: String,
  pub groups: Vec<GammaGroup>,
}

impl GammaUser {
  pub fn from_user_and_groups(
    user: gamma_rust_client::api::GammaUser,
    groups: Vec<gamma_rust_client::api::GammaUserGroup>,
  ) -> Self {
    Self {
      id: user.id,
      cid: user.cid,
      nick: user.nick,
      first_name: user.first_name,
      last_name: user.last_name,
      groups: groups.into_iter().map(|g| g.into()).collect(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizedUser {
  pub user_id: Uuid,
}

impl From<gamma_rust_client::oauth::GammaOpenIDUser> for AuthorizedUser {
  fn from(value: gamma_rust_client::oauth::GammaOpenIDUser) -> Self {
    Self {
      user_id: value.user_id,
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GammaGroup {
  pub id: Uuid,
  pub name: String,
  pub pretty_name: String,
  pub super_group: GammaSuperGroup,
  pub post: GammaGroupPost,
}

impl From<gamma_rust_client::api::GammaUserGroup> for GammaGroup {
  fn from(value: gamma_rust_client::api::GammaUserGroup) -> Self {
    Self {
      id: value.id,
      name: value.name,
      pretty_name: value.pretty_name,
      super_group: value.super_group.into(),
      post: value.post.into(),
    }
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GammaSuperGroup {
  pub id: Uuid,
  pub name: String,
  pub pretty_name: String,
  #[serde(rename = "type")]
  pub group_type: GammaGroupType,
  pub sv_description: String,
  pub en_description: String,
}

impl From<gamma_rust_client::api::GammaSuperGroup> for GammaSuperGroup {
  fn from(value: gamma_rust_client::api::GammaSuperGroup) -> Self {
    Self {
      id: value.id,
      name: value.name,
      pretty_name: value.pretty_name,
      group_type: value.group_type.into(),
      sv_description: value.sv_description,
      en_description: value.en_description,
    }
  }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename = "camelCase")]
pub enum GammaGroupType {
  Society,
  Functionaries,
  Committee,
  Alumni,
  Other(String),
}

impl From<gamma_rust_client::api::GammaSuperGroupType> for GammaGroupType {
  fn from(value: gamma_rust_client::api::GammaSuperGroupType) -> Self {
    match value {
      gamma_rust_client::api::GammaSuperGroupType::Alumni => Self::Alumni,
      gamma_rust_client::api::GammaSuperGroupType::Committee => Self::Committee,
      gamma_rust_client::api::GammaSuperGroupType::Society => Self::Society,
      gamma_rust_client::api::GammaSuperGroupType::Functionaries => Self::Functionaries,
      gamma_rust_client::api::GammaSuperGroupType::Admin => Self::Other("Admin".into()),
      gamma_rust_client::api::GammaSuperGroupType::Other(v) => Self::Other(v),
    }
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GammaGroupPost {
  pub id: Uuid,
  pub version: i32,
  pub sv_name: String,
  pub en_name: String,
}

impl From<gamma_rust_client::api::GammaPost> for GammaGroupPost {
  fn from(value: gamma_rust_client::api::GammaPost) -> Self {
    Self {
      id: value.id,
      version: value.version,
      sv_name: value.sv_name,
      en_name: value.en_name,
    }
  }
}
