use async_trait::async_trait;
use redis::{aio::MultiplexedConnection, SetOptions};
use redis::AsyncCommands;

use crate::common::infrastructure::redis::get_redis_db;
use crate::domain::redis::RedisCliImpl;
use crate::utils::error::Result;

#[allow(dead_code)]
async fn get(key: String) -> Result<String> {
    let mut c = get_redis_db()?;
    let value: String = c.get(key).await?;
    Ok(value)
}

#[allow(dead_code)]
pub struct RedisCLi<'a> {
    new_db: &'a mut MultiplexedConnection,
}

#[allow(dead_code)]
impl<'a> RedisCLi<'a> {
    pub(crate) fn new(db: &'a mut MultiplexedConnection) -> Self {
        Self { new_db: db }
    }
}

#[async_trait]
impl<'a> RedisCliImpl for RedisCLi<'a> {
    async fn get(&mut self, key: String) -> Result<String> {
        let value: String = self.new_db.get(key).await?;
        Ok(value)
    }

    async fn set(&mut self, key: String, value: String, option: SetOptions) -> Result<()> {
        let _ = self.new_db.set_options(key, value, option).await?;
        Ok(())
    }
}
