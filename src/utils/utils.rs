use crate::utils::error::Result;
use sqlx::types::uuid::Uuid;
use uuid;

#[allow(dead_code)]
pub fn uuid_new() -> Result<Uuid> {
    let u = uuid::Uuid::new_v4().to_string();

    Ok(Uuid::parse_str(u.as_str())?)
}

#[allow(dead_code)]
pub fn parse_uuid(v: String) -> Result<Uuid> {
    Ok(Uuid::parse_str(&v)?)
}
