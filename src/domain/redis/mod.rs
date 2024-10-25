use async_trait::async_trait;
use redis::SetOptions;

use crate::utils::error::Result;

#[allow(dead_code)]
#[async_trait]
pub trait RedisCliImpl: Send + Sync {
    async fn get(&mut self, key: String) -> Result<String>;
    async fn set(&mut self, key: String, value: String, option: SetOptions) -> Result<()>;
}
