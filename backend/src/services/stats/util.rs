use chrono::{DateTime, Duration, Local, NaiveDate, TimeZone, Weekday};
use std::collections::HashMap;
use uuid::Uuid;

use crate::models::UserSession;

use super::{DateTimeRange, Stat};

pub fn map_sessions(sessions: Vec<UserSession>) -> HashMap<Uuid, Vec<DateTimeRange>> {
  let mut sessions_map = HashMap::new();
  for session in sessions {
    sessions_map
      .entry(session.user_id)
      .or_insert_with(Vec::new)
      .push((
        session.start_time.with_timezone(&Local),
        session.end_time.with_timezone(&Local),
      ));
  }

  sessions_map
}

pub fn calculate_stats(
  user_sessions: &HashMap<Uuid, Vec<DateTimeRange>>,
  range_start_time: DateTime<Local>,
  range_end_time: DateTime<Local>,
) -> HashMap<Uuid, Stat> {
  user_sessions
    .iter()
    .map(|(&user_id, sessions)| {
      let session_duration =
        sessions
          .iter()
          .fold(Duration::zero(), |prev, &(start_time, end_time)| {
            // Don't count session time outside of the range
            let session_minutes = if end_time > range_end_time {
              range_end_time - start_time
            } else if start_time < range_start_time {
              end_time - range_start_time
            } else {
              end_time - start_time
            };

            prev + session_minutes
          });

      (
        user_id,
        Stat {
          user_id,
          duration_ms: session_duration.num_milliseconds(),
        },
      )
    })
    .collect()
}

pub fn join_stats(stats: &mut HashMap<Uuid, Stat>, other_stats: &HashMap<Uuid, Stat>) {
  for (user_id, stat) in other_stats {
    stats
      .entry(*user_id)
      .and_modify(|s| {
        s.duration_ms += stat.duration_ms;
      })
      .or_insert_with(|| stat.clone());
  }
}

pub fn year_time_bounds(year: i32) -> (DateTime<Local>, DateTime<Local>) {
  let start_time = Local.with_ymd_and_hms(year, 1, 1, 0, 0, 0).unwrap();
  let end_time = Local.with_ymd_and_hms(year, 12, 31, 23, 59, 59).unwrap();
  (start_time, end_time)
}

pub fn month_time_bounds(year: i32, month: u32) -> (DateTime<Local>, DateTime<Local>) {
  let start_time = Local.with_ymd_and_hms(year, month, 1, 0, 0, 0).unwrap();
  let end_time = if month == 12 {
    Local.with_ymd_and_hms(year + 1, 1, 1, 0, 0, 0).unwrap()
  } else {
    Local.with_ymd_and_hms(year, month + 1, 1, 0, 0, 0).unwrap()
  } - Duration::seconds(1);
  (start_time, end_time)
}

pub fn day_time_bounds(year: i32, month: u32, day: u32) -> (DateTime<Local>, DateTime<Local>) {
  let start_time = Local.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap();
  let end_time = Local
    .with_ymd_and_hms(year, month, day, 23, 59, 59)
    .unwrap();
  (start_time, end_time)
}

pub fn month_date_bounds(year: i32, month: u32) -> (NaiveDate, NaiveDate) {
  let start_time = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
  let end_time = if month == 12 {
    Local.with_ymd_and_hms(year + 1, 1, 1, 23, 59, 59)
  } else {
    Local.with_ymd_and_hms(year, month + 1, 1, 0, 0, 0)
  }
  .unwrap()
    - Duration::seconds(1);
  let end_time = end_time.date_naive();
  (start_time, end_time)
}

pub fn week_date_bounds(year: i32, week: u32) -> (NaiveDate, NaiveDate) {
  let start_time = NaiveDate::from_isoywd_opt(year, week, Weekday::Mon).unwrap();
  let end_time = NaiveDate::from_isoywd_opt(year, week, Weekday::Sun).unwrap();
  (start_time, end_time)
}

pub fn day_date_bounds(year: i32, month: u32, day: u32) -> (NaiveDate, NaiveDate) {
  let start_time = NaiveDate::from_ymd_opt(year, month, day).unwrap();
  let end_time = NaiveDate::from_ymd_opt(year, month, day).unwrap();
  (start_time, end_time)
}
