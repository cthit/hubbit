use chrono::{DateTime, Local, TimeZone};
use lazy_static::lazy_static;

lazy_static! {
  pub static ref MIN_DATETIME: DateTime<Local> =
    Local.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
  pub static ref MAX_DATETIME: DateTime<Local> =
    Local.with_ymd_and_hms(2099, 12, 31, 23, 59, 59).unwrap();
}
