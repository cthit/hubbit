use async_graphql::{guard::Guard, Context, InputObject, Object, SimpleObject};
use chrono::{DateTime, Duration, Local, Utc};
use log::error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
  models::{GammaUser, UserSession},
  repositories::{device::DeviceRepository, user_session::UserSessionRepository},
  services::{hour_stats::HourStatsService, user::UserService},
  utils::{MAX_DATETIME, MIN_DATETIME},
};

use super::{device::Device, AuthGuard, HubbitSchemaError, HubbitSchemaResult};
use crate::config::Config;
use crate::repositories::study_year::StudyYearRepository;
use crate::services::stats::{Stat, StatsService};

#[derive(Default)]
pub struct UserQuery;

#[derive(InputObject)]
pub struct UserUniqueInput {
  id: Option<Uuid>,
  cid: Option<String>,
}

#[derive(PartialEq, SimpleObject)]
pub struct Group {
  name: String,
  pretty_name: String,
}

#[Object]
impl UserQuery {
  #[graphql(guard(AuthGuard()))]
  pub async fn user(
    &self,
    context: &Context<'_>,
    input: UserUniqueInput,
  ) -> HubbitSchemaResult<User> {
    let user_service = context.data_unchecked::<UserService>();
    let user = if let Some(id) = input.id {
      user_service.get_by_id(id, false).await.map_err(|e| {
        error!("[Schema error] {:?}", e);
        HubbitSchemaError::InternalError
      })?
    } else if let Some(cid) = input.cid {
      user_service.get_by_cid(cid).await.map_err(|e| {
        error!("[Schema error] {:?}", e);
        HubbitSchemaError::InternalError
      })?
    } else {
      return Err(HubbitSchemaError::InvalidInput);
    };
    Ok(User { id: user.id })
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
  pub id: Uuid,
}

#[Object]
impl User {
  async fn id(&self) -> Uuid {
    self.id
  }

  async fn cid(&self, context: &Context<'_>) -> HubbitSchemaResult<String> {
    let user_service = context.data_unchecked::<UserService>();
    let user = user_service
      .get_by_id(self.id, false)
      .await
      .map_err(|_| HubbitSchemaError::InternalError)?;
    Ok(user.cid)
  }

  async fn nick(&self, context: &Context<'_>) -> HubbitSchemaResult<String> {
    let user_service = context.data_unchecked::<UserService>();
    let user = user_service
      .get_by_id(self.id, false)
      .await
      .map_err(|_| HubbitSchemaError::InternalError)?;
    Ok(user.nick)
  }

  async fn first_name(&self, context: &Context<'_>) -> HubbitSchemaResult<String> {
    let user_service = context.data_unchecked::<UserService>();
    let user = user_service
      .get_by_id(self.id, false)
      .await
      .map_err(|_| HubbitSchemaError::InternalError)?;
    Ok(user.first_name)
  }

  async fn last_name(&self, context: &Context<'_>) -> HubbitSchemaResult<String> {
    let user_service = context.data_unchecked::<UserService>();
    let user = user_service
      .get_by_id(self.id, false)
      .await
      .map_err(|_| HubbitSchemaError::InternalError)?;
    Ok(user.last_name)
  }

  async fn avatar_url(&self, context: &Context<'_>) -> HubbitSchemaResult<String> {
    let user_service = context.data_unchecked::<UserService>();
    let user = user_service
      .get_by_id(self.id, false)
      .await
      .map_err(|_| HubbitSchemaError::InternalError)?;
    Ok(user.avatar_url)
  }

  async fn groups(&self, context: &Context<'_>) -> HubbitSchemaResult<Vec<Group>> {
    let user_service = context.data_unchecked::<UserService>();
    let config = context.data_unchecked::<Config>();
    let user = user_service
      .get_by_id(self.id, false)
      .await
      .map_err(|_| HubbitSchemaError::InternalError)?;
    let mut groups = user
      .groups
      .into_iter()
      .filter(|group| {
        group.active
          && (config.group_whitelist.is_empty()
            || config.group_whitelist.contains(&group.super_group.name))
      })
      .map(|group| Group {
        name: group.super_group.name,
        pretty_name: group.super_group.pretty_name,
      })
      .collect::<Vec<_>>();
    groups.dedup();
    Ok(groups)
  }

  async fn hour_stats(&self, context: &Context<'_>) -> HubbitSchemaResult<Vec<u32>> {
    let hour_stats_service = context.data_unchecked::<HourStatsService>();
    Ok(
      hour_stats_service
        .get_for_user(self.id)
        .await
        .map_err(|_| HubbitSchemaError::InternalError)?,
    )
  }

  async fn recent_sessions(&self, context: &Context<'_>) -> HubbitSchemaResult<Vec<Session>> {
    let user_session_repo = context.data_unchecked::<UserSessionRepository>();
    let sessions = user_session_repo
      .get_range_for_user(*MIN_DATETIME, *MAX_DATETIME, self.id)
      .await
      .map_err(|_| HubbitSchemaError::InternalError)?;
    Ok(
      sessions
        .iter()
        .map(|session| Session {
          start_time: session.start_time,
          end_time: session.end_time,
        })
        .take(10)
        .collect(),
    )
  }

  async fn longest_session(&self, context: &Context<'_>) -> HubbitSchemaResult<Option<Session>> {
    let user_session_repo = context.data_unchecked::<UserSessionRepository>();
    let sessions = user_session_repo
      .get_range_for_user(*MIN_DATETIME, *MAX_DATETIME, self.id)
      .await
      .map_err(|_| HubbitSchemaError::InternalError)?;
    let mut longest_session: Option<UserSession> = None;
    for session in sessions {
      if let Some(longest_session_inner) = &longest_session {
        if session.end_time - session.start_time
          > longest_session_inner.end_time - longest_session_inner.start_time
        {
          longest_session = Some(session);
        }
      } else {
        longest_session = Some(session);
      }
    }

    Ok(longest_session.map(|session| Session {
      start_time: session.start_time,
      end_time: session.end_time,
    }))
  }

  async fn total_time_seconds(&self, context: &Context<'_>) -> HubbitSchemaResult<i64> {
    let user_session_repo = context.data_unchecked::<UserSessionRepository>();
    let sessions = user_session_repo
      .get_range_for_user(*MIN_DATETIME, *MAX_DATETIME, self.id)
      .await
      .map_err(|_| HubbitSchemaError::InternalError)?;

    let duration_ms = sessions.iter().fold(0, |prev, cur| {
      prev + (cur.end_time - cur.start_time).num_milliseconds()
    });

    Ok(duration_ms / 1000)
  }

  async fn average_time_per_day(&self, context: &Context<'_>) -> HubbitSchemaResult<i64> {
    let user_session_repo = context.data_unchecked::<UserSessionRepository>();
    let sessions = user_session_repo
      .get_range_for_user(*MIN_DATETIME, *MAX_DATETIME, self.id)
      .await
      .map_err(|_| HubbitSchemaError::InternalError)?;

    let duration_ms = sessions.iter().fold(0, |prev, cur| {
      prev + (cur.end_time - cur.start_time).num_milliseconds()
    });

    let all_time_seconds = duration_ms / 1000;
    let first_day = user_session_repo
      .get_first_entry_day(self.id)
      .await
      .map_err(|_| HubbitSchemaError::InternalError)?
      .ok_or(HubbitSchemaError::InternalError)?;

    let today = Local::now();
    let num_days_since_first = today.signed_duration_since(first_day.start_time).num_days();

    Ok(
      all_time_seconds
        .checked_div(num_days_since_first)
        .unwrap_or(0),
    )
  }

  async fn time_today_seconds(&self, context: &Context<'_>) -> HubbitSchemaResult<i64> {
    let user_session_repo = context.data_unchecked::<UserSessionRepository>();
    let today = Local::now().date();
    let today_start = today.and_hms(0, 0, 0);
    let today_end = (today_start + Duration::days(1)) - Duration::seconds(1);

    let sessions = user_session_repo
      .get_range_for_user(today_start, today_end, self.id)
      .await
      .map_err(|_| HubbitSchemaError::InternalError)?;

    let duration_ms = sessions.iter().fold(0, |prev, curr| {
      let start_time = if curr.start_time.date().eq(&today) {
        DateTime::<Local>::from(curr.start_time)
      } else {
        today_start
      };
      let end_time = if curr.end_time.date().eq(&today) {
        DateTime::<Local>::from(curr.end_time)
      } else {
        today_end
      };
      prev + (end_time - start_time).num_milliseconds()
    });

    Ok(duration_ms / 1000)
  }

  pub async fn devices(&self, context: &Context<'_>) -> HubbitSchemaResult<Vec<Device>> {
    let auth_user = context
      .data::<GammaUser>()
      .map_err(|_| HubbitSchemaError::NotLoggedIn)?;
    if self.id != auth_user.id {
      return Err(HubbitSchemaError::NotAuthorized);
    }

    let device_repo = context.data_unchecked::<DeviceRepository>();
    let devices = device_repo
      .get_for_user(self.id)
      .await
      .map_err(|_| HubbitSchemaError::InternalError)?;
    Ok(
      devices
        .into_iter()
        .map(|device| Device { id: device.id })
        .collect(),
    )
  }

  pub async fn curr_alltime_position(
    &self,
    context: &Context<'_>,
  ) -> HubbitSchemaResult<Option<i64>> {
    let stats_service = context.data_unchecked::<StatsService>();
    let stats = stats_service.get_alltime().await.map_err(|e| {
      error!("[Schema error] {:?}", e);
      HubbitSchemaError::InternalError
    })?;

    let mut sort_stats = stats.into_iter().map(|s| s.1).collect::<Vec<Stat>>();

    sort_stats.sort_by_key(|s| -s.duration_ms);

    for (index, stat) in sort_stats.into_iter().enumerate() {
      if stat.user_id == self.id {
        return Ok(Some((index + 1) as i64));
      }
    }

    Ok(None)
  }

  pub async fn curr_study_year_position(
    &self,
    context: &Context<'_>,
  ) -> HubbitSchemaResult<Option<i64>> {
    let study_year_repo = context.data_unchecked::<StudyYearRepository>();
    let curr_year = study_year_repo
      .get_current()
      .await
      .map_err(|e| {
        error!("[Schema error] {:?}", e);
        HubbitSchemaError::InternalError
      })?
      .year;

    let stats_service = context.data_unchecked::<StatsService>();
    let stats = match stats_service.get_study_year(curr_year).await {
      Ok(s) => s,
      Err(e) => {
        error!("[Schema error] {:?}", e);
        return Err(HubbitSchemaError::InternalError);
      }
    };

    let mut sort_stats = stats.into_iter().map(|s| s.1).collect::<Vec<Stat>>();
    sort_stats.sort_by_key(|s| -s.duration_ms);

    for (index, stat) in sort_stats.into_iter().enumerate() {
      if stat.user_id == self.id {
        return Ok(Some((index + 1) as i64));
      }
    }

    Ok(None)
  }
}

#[derive(SimpleObject)]
pub struct Session {
  start_time: DateTime<Utc>,
  end_time: DateTime<Utc>,
}
