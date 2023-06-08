use crate::{
    domain::repo_info::RepoInfo,
    utils::{
        error::Result,
        time::{sub_now, timestamp_to_date},
    },
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RepoInfoDTO {
    pub uuid: String,
    pub owner: String,
    pub repo: String,
    pub branch: String,
    pub status: String,
    pub last_commit: String,
    pub timeout: i32,
    pub since: i64,
    pub modified_time: String,
}

impl RepoInfoDTO {
    pub fn from(v: RepoInfo) -> Result<RepoInfoDTO> {
        Ok(RepoInfoDTO {
            uuid: v.uuid.to_string(),
            owner: v.owner,
            repo: v.repo,
            branch: v.branch,
            status: v.status,
            last_commit: v.last_commit,
            timeout: v.timeout,
            since: sub_now(v.modified_time),
            modified_time: timestamp_to_date(v.modified_time, "")?,
        })
    }
}

#[derive(Clone)]
pub struct CmdToRepoInfo {
    pub owner: String,
    pub repo: String,
    pub branch: String,
    pub commit: String,
}
