use crate::utils::error::{Error, Result};
use once_cell::sync::OnceCell;
use sqlx::{Connection, Pool};
use sqlx::mysql::{MySqlPoolOptions, MySql};
use std::sync::{Arc, RwLock};
use std::time::Duration;
use chrono::Duration as chDuration;

pub type MysqlDB = Pool<MySql>;

static DB_CLI: OnceCell<MysqlDB> = OnceCell::new();

pub async fn init_mysql_db(v: Arc<RwLock<config::Config>>) -> Result<()> {
    let connect_url = v
        .read()?
        .get_string("mysql.connect_url")
        .expect("database connection url")
        .to_string();

    if connect_url.is_empty() {
        return Err(Error::ConfigError(String::from("connect mysql url is empty")));
    }

    let max_connections: u32 = v
        .read()?
        .get_int("mysql.max_connections")
        .expect("connect database max connections") as u32;

    let lift_time = v
        .read()?
        .get_int("mysql.max_life_time")
        .expect("connect database max life time");

    let db = MySqlPoolOptions::new()
        .max_connections(max_connections)
        .max_lifetime(Duration::from_secs(
            chDuration::minutes(lift_time).num_seconds() as u64,
        )).
        connect(&connect_url).
        await?;

    DB_CLI.set(db).expect("db pool configured");

    Ok(())
}

pub fn get_db() -> Result<MysqlDB> {
    match DB_CLI.get() {
        Some(db) => Ok(db.clone()),
        None => Err(Error::DataBaseError(String::from("no db pool"))),
    }
}
