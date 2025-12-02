use {
    crate::utils::error::{Error, Result},
    chrono::Duration as chDuration,
    once_cell::sync::OnceCell,
    sea_orm::{ConnectOptions, Database, DatabaseConnection},
    sqlx::mysql::{MySql, MySqlPoolOptions},
    sqlx::Pool,
    std::sync::{Arc, RwLock},
    std::time::Duration,
};

pub type MysqlDB = Pool<MySql>;

static DB_CLI: OnceCell<MysqlDB> = OnceCell::new();

static DB_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::new();

pub async fn init_mysql_db(v: Arc<RwLock<config::Config>>) -> Result<()> {
    let connect_url = v
        .read()?
        .get_string("mysql.connect_url")
        .expect("database connection url")
        .to_string();

    if connect_url.is_empty() {
        return Err(Error::ConfigError(String::from(
            "connect mysql url is empty",
        )));
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
        ))
        .connect(&connect_url)
        .await?;

    DB_CLI.set(db).expect("db pool configured");

    let mut opt = ConnectOptions::new(connect_url);
    opt.max_connections(max_connections)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));
    // .sqlx_logging(true)
    // .sqlx_logging_level(log::LevelFilter::Info);

    DB_CONNECTION
        .set(Database::connect(opt).await?)
        .expect("database connection configured");
    get_db_connection()?
        .ping()
        .await
        .expect("database connection failed");

    Ok(())
}

#[allow(dead_code)]
pub fn get_db() -> Result<MysqlDB> {
    match DB_CLI.get() {
        Some(db) => Ok(db.clone()),
        None => Err(Error::DataBaseError(String::from("no db pool"))),
    }
}

pub fn get_db_connection() -> Result<&'static DatabaseConnection> {
    match DB_CONNECTION.get() {
        None => Err(Error::NewDataBaseError(
            "failed to get database pool".to_string(),
        )),
        Some(pool) => Ok(pool),
    }
}
