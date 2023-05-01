use crate::utils::error::{Error, Result};
use chrono::prelude::*;

pub fn timestamp_to_date(i: i64, format: &str) -> Result<String> {
    let mut f = format;
    if f.is_empty() {
        f = "%Y-%m-%d %H:%M:%S"
    }

    match NaiveDateTime::from_timestamp_opt(i, 0) {
        None => Err(Error::DateError(String::from("parse time failed"))),
        Some(s) => Ok(DateTime::<Utc>::from_local(s, Utc).format(f).to_string()),
    }
}

pub fn now() -> i64 {
    Local::now().timestamp()
}

pub fn sub_now(i: i64) -> i64 {
    now() - i
}
