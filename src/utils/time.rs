use chrono::prelude::*;

use crate::utils::error::{Error, Result};

pub fn timestamp_to_date(i: i64, format: &str) -> Result<String> {
    let mut f = format;
    if f.is_empty() {
        f = "%Y-%m-%d %H:%M:%S"
    }

    match DateTime::from_timestamp(i, 0) {
        None => Err(Error::DateError(String::from("parse time failed"))),
        Some(s) => Ok(s.format(f).to_string()),
    }
}

pub fn now() -> i64 {
    Utc::now().timestamp()
}

pub fn sub_now(i: i64) -> i64 {
    now() - i
}
