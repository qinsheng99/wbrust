use redis::AsyncCommands;

use crate::common::infrastructure::redis::get_redis_db;
use crate::utils::error::Result;

#[allow(dead_code)]
async fn get(key: String) -> Result<String> {
    let mut c = get_redis_db()?;
    let value: String = c.get(key).await?;
    Ok(value)
}
