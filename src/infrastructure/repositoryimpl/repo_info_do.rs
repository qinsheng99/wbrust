use crate::domain::repo_info::RepoInfo;
use sqlx::types::uuid::Uuid;
use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct RepoInfoDO {
    pub uuid: Uuid,
    pub owner: String,
    pub repo: String,
    pub branch: String,
    pub status: String,
    pub last_commit: String,
    pub timeout: i64,
    pub modified_time: i64,
}

impl From<RepoInfoDO> for RepoInfo {
    fn from(v: RepoInfoDO) -> RepoInfo {
        RepoInfo {
            uuid: v.uuid,
            repo: v.repo,
            owner: v.owner,
            branch: v.branch,
            status: v.status,
            last_commit: v.last_commit,
            timeout: v.timeout,
            modified_time: v.modified_time,
        }
    }
}
