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
  #[serde(rename = "sub")]
  pub id: Uuid,
  pub cid: String,
  #[serde(rename = "nickname")]
  pub nick: String,
  #[serde(rename = "given_name")]
  pub first_name: String,
  #[serde(rename = "family_name")]
  pub last_name: String,
  #[serde(rename = "picture")]
  pub avatar_url: String,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "camelCase")]
pub enum GammaGroupType {
  Society,
  Functionaries,
  Committee,
  Alumni,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GammaGroupPost {
  pub id: Uuid,
  pub version: u32,
  pub sv_name: String,
  pub en_name: String,
}
