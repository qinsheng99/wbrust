use crate::utils::error::{Error, Result};
use chrono::prelude::*;

pub fn timestamp_to_date(i: i64, format: &str) -> Result<String> {
    let mut f = format;
    if f.is_empty() {
        f = "%Y-%m-%d %H:%M:%S"
    }
    match NaiveDateTime::from_timestamp_micros(i) {
        None => Err(Error::DateError(String::from("parse time failed"))),
        Some(s) => Ok(s.format(f).to_string()),
    }
}
