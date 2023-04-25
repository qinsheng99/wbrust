use crate::utils::error::{Error, Result};
use chrono::Duration as chDuration;
use once_cell::sync::OnceCell;
use sqlx::postgres::{PgPoolOptions, Postgres};
use sqlx::Pool;
use std::sync::{Arc, RwLock};
use std::time::Duration;

pub type PgDB = Pool<Postgres>;

static DB_CLI: OnceCell<PgDB> = OnceCell::new();

pub async fn init_db(v: Arc<RwLock<config::Config>>) -> Result<()> {
    let connect_url = v
        .read()?
        .get_string("postgresql.connect_url")
        .expect("database connection url")
        .to_string();

    if connect_url.is_empty() {
        return Err(Error::ConfigError(String::from("connect url is empty")));
    }

    let max_connections: u32 = v
        .read()?
        .get_int("postgresql.max_connections")
        .expect("connect database max connections") as u32;

    let lift_time = v
        .read()?
        .get_int("postgresql.max_life_time")
        .expect("connect database max life time");

    let pg = PgPoolOptions::new()
        .max_connections(max_connections)
        .max_lifetime(Duration::from_secs(
            chDuration::minutes(lift_time).num_seconds() as u64,
        ))
        .connect(&connect_url)
        .await?;

    DB_CLI.set(pg).expect("db pool configured");

    Ok(())
}
