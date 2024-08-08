use std::sync::{Arc, RwLock};

use config::Config;
use once_cell::sync::OnceCell;
use redis::{
    aio::MultiplexedConnection, Client, ConnectionAddr, ConnectionInfo, IntoConnectionInfo,
    RedisResult,
};

use crate::utils::error::{Error, Result};

// pub type RedisDB = MultiplexedConnection;

static REDIS_DB_CLI: OnceCell<MultiplexedConnection> = OnceCell::new();

struct RedisParam {
    host: String,
    port: u16,
    password: String,
}

impl IntoConnectionInfo for RedisParam {
    fn into_connection_info(self) -> RedisResult<ConnectionInfo> {
        let mut info = ConnectionInfo {
            addr: ConnectionAddr::Tcp(self.host, self.port),
            redis: Default::default(),
        };

        if !self.password.is_empty() {
            info.redis.password = Some(self.password)
        }
        Ok(info)
    }
}

pub async fn init_redis(cfg: Arc<RwLock<Config>>) -> Result<()> {
    let host = cfg.read()?.get_string("redis.host").unwrap();

    let port = cfg.read()?.get_int("redis.port").unwrap() as u16;

    let password = cfg.read()?.get_string("redis.password").unwrap();

    let param = RedisParam {
        host,
        port,
        password,
    };

    let client = Client::open(param).unwrap();
    let stearm = client.get_multiplexed_async_connection().await?;

    if let Some(_err) = REDIS_DB_CLI.set(stearm).err() {
        Error::RedisError(String::from("set redis failed"));
    }

    Ok(())
}

#[allow(dead_code)]
pub fn get_redis_db() -> Result<MultiplexedConnection> {
    match REDIS_DB_CLI.get() {
        None => Err(Error::RedisError(String::from("redis cli is none"))),
        Some(db) => Ok(db.clone()),
    }
}
