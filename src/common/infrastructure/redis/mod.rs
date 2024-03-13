use crate::utils::error::{Error, Result};
use config::Config;
use once_cell::sync::OnceCell;
use redis::{Client, Connection, ConnectionAddr, ConnectionInfo, IntoConnectionInfo, RedisResult};
use std::sync::{Arc, RwLock};

pub type RedisDB = Connection;

static REDIS_DB_CLI: OnceCell<RedisDB> = OnceCell::new();

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

    let conn = client.get_connection()?;

    if let Some(_err) = REDIS_DB_CLI.set(conn).err() {
        Error::RedisError(String::from("set redis failed"));
    }

    Ok(())
}
