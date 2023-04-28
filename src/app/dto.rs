use crate::{
    domain::repo_info::RepoInfo,
    utils::{error::Result, time::timestamp_to_date},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct RepoInfoDTO {
    pub uuid: String,
    pub owner: String,
    pub repo: String,
    pub branch: String,
    pub status: String,
    pub last_commit: String,
    pub timeout: i64,
    pub modified_time: String,
}

impl RepoInfoDTO {
    pub fn from(v: RepoInfo) -> Result<RepoInfoDTO> {
        Ok(RepoInfoDTO {
            uuid: "".to_string(),
            owner: "".to_string(),
            repo: "".to_string(),
            branch: "".to_string(),
            status: "".to_string(),
            last_commit: "".to_string(),
            timeout: 0,
            modified_time: timestamp_to_date(v.modified_time, "")?,
        })
    }
}
