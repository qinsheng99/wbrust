use crate::app::dto::{CmdToListQuery, CmdToRepoInfo};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RepoInfoRequest {
    owner: String,
    repo: String,
    branch: String,
    commit: String,
}

impl RepoInfoRequest {
    pub fn to_cmd(self) -> CmdToRepoInfo {
        CmdToRepoInfo {
            owner: self.owner,
            repo: self.repo,
            branch: self.branch,
            commit: self.commit,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ListQuery {
    pub page: i64,
    pub size: i64,
}

impl ListQuery {
    pub fn to_cmd(self) -> CmdToListQuery {
        CmdToListQuery {
            page: self.page,
            size: self.size,
        }
    }
}
