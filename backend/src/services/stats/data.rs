use chrono::{DateTime, Datelike, Local, NaiveDate};

use crate::{
  error::HubbitResult,
  services::util::{redis_get, redis_set},
  utils::{MAX_DATETIME, MIN_DATETIME},
};

use super::{
  util::{calculate_stats, map_sessions},
  Stats, StatsService,
};

impl StatsService {
  pub(super) async fn get_range_fresh(
    &self,
    start_time: DateTime<Local>,
    end_time: DateTime<Local>,
  ) -> HubbitResult<Stats> {
    let sessions = self
      .user_session_repo
      .get_range(start_time, end_time)
      .await?;
    let sessions = map_sessions(sessions);
    Ok(calculate_stats(&sessions, start_time, end_time))
  }

  pub async fn get_earliest_date(&self) -> HubbitResult<NaiveDate> {
    let mut earliest_date_lock = self.earliest_date.lock().await;
    if let Some(earliest_date) = *earliest_date_lock {
      return Ok(earliest_date);
    }

    if let Ok(earliest_date) = redis_get(self.redis_pool.clone(), "earliest_date").await {
      return Ok(earliest_date);
    }

    let sessions = self
      .user_session_repo
      .get_range(*MIN_DATETIME, *MAX_DATETIME)
      .await?;
    let earliest_date = sessions
      .iter()
      .fold(*MAX_DATETIME, |prev, cur| {
        prev.min(cur.start_time.with_timezone(&Local))
      })
      .date_naive()
      .with_day(1)
      .expect("Could not set day to 1")
      .with_month(1)
      .expect("Could not set month to 1");

    // Only save the earliest date if there are any sessions, otherwise it
    // saves the earliest date as the MAX_DATETIME, which screws everything
    if !sessions.is_empty() {
      *earliest_date_lock = Some(earliest_date);

      let redis_pool = self.redis_pool.clone();
      let earliest_date_clone = earliest_date;
      tokio::spawn(async move {
        redis_set(redis_pool, "earliest_date".to_owned(), earliest_date_clone).await
      });
    }

    Ok(earliest_date)
  }
}
