use crate::{
    app::dto::CmdToRepoInfo,
    domain::repo_info::RepoInfo,
    utils::{error::Result, time::now, utils::uuid_new},
};

use sqlx::types::uuid::Uuid;

use sqlx::FromRow;

const FREE: &'static str = "free";
const TIME_OUT: i32 = 30;

#[derive(FromRow, Debug)]
pub struct RepoInfoDO {
    pub uuid: Uuid,
    pub owner: String,
    pub repo: String,
    pub branch: String,
    pub status: String,
    pub last_commit: String,
    pub timeout: i32,
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

pub async fn to_repo_info_do(v: CmdToRepoInfo) -> Result<RepoInfoDO> {
    Ok(RepoInfoDO {
        uuid: uuid_new()?,
        owner: v.owner,
        repo: v.repo,
        branch: v.branch,
        status: FREE.parse()?,
        last_commit: v.commit,
        timeout: TIME_OUT,
        modified_time: now(),
    })
}

#[derive(FromRow, Debug)]
pub struct Total {
    pub total: i64,
}
