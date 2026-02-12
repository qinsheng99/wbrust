use {
    crate::{
        app::dto::{CmdToListQuery, CmdToRepoInfo},
        utils::error::Result,
    },
    async_trait::async_trait,
    sqlx::types::uuid::Uuid,
};

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

#[allow(dead_code)]
pub struct RepoInfoModel {
    pub id: u64,
    pub owner: String,
    pub repo: String,
    pub branch: String,
    pub status: String,
    pub last_commit: String,
    pub timeout: i32,
    pub modified_time: u64,
}

#[derive(Default)]
pub struct ListRepoInfo {
    pub repo_list: Vec<RepoInfo>,
    pub total: i64,
}

#[async_trait]
pub trait RepoImpl: Send + Sync {
    async fn repo_detail_info(&self, id: String) -> Result<RepoInfo>;
    async fn add(&self, v: CmdToRepoInfo) -> Result<()>;
    async fn total(&self, v: CmdToListQuery) -> Result<i64>;
    async fn list(&self, v: CmdToListQuery) -> Result<ListRepoInfo>;
}

#[async_trait]
#[allow(dead_code)]
pub trait NewRepoInfoImpl: Send + Sync {
    async fn repo_detail_info_for_sea(&self, id: u64) -> Result<RepoInfoModel>;
    async fn rq(&self) -> Result<()>;
}
