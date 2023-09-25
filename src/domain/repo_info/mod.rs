use crate::app::dto::{CmdToListQuery, CmdToRepoInfo};
use crate::utils::error::Result;
use async_trait::async_trait;
use sqlx::types::uuid::Uuid;

pub struct RepoInfo {
    pub uuid: Uuid,
    pub owner: String,
    pub repo: String,
    pub branch: String,
    pub status: String,
    pub last_commit: String,
    pub timeout: i32,
    pub modified_time: i64,
}

#[async_trait]
pub trait RepoImpl: Send + Sync {
    async fn repo_detail_info(&self, id: String) -> Result<RepoInfo>;
    async fn add(&self, v: CmdToRepoInfo) -> Result<()>;
    async fn total(&self, v: CmdToListQuery) -> Result<i64>;
}
