use crate::{
    domain::repo_info::{ListRepoInfo, RepoInfo},
    utils::{
        error::{Error, Result},
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
            modified_time: timestamp_to_date(v.modified_time.clone(), "")?,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct RepoInfoListDTO {
    pub repos: Vec<RepoInfoDTO>,
    pub total: i64,
}

impl RepoInfoListDTO {
    pub fn from(v: ListRepoInfo) -> Result<RepoInfoListDTO> {
        let mut repos: Vec<RepoInfoDTO> = vec![];

        for item in v.repo_list {
            if let Ok(i) = RepoInfoDTO::from(item) {
                repos.push(i);
                continue;
            } else {
                return Err(Error::ParseError(String::from("parse repo info failed")));
            }
        }

        Ok(RepoInfoListDTO {
            repos,
            total: v.total,
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

#[derive(Clone)]
pub struct CmdToListQuery {
    pub page: i64,
    pub size: i64,
}
