use crate::app::dto::CmdToRepoInfo;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RepoInfoRequest {
    owner: String,
    repo: String,
    branch: String,
    commit: String,
}

impl RepoInfoRequest {
    pub fn to_cmd(&self) -> CmdToRepoInfo {
        CmdToRepoInfo {
            owner: self.owner.clone(),
            repo: self.repo.clone(),
            branch: self.branch.clone(),
            commit: self.commit.clone(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ListQuery {
    pub page: i64,
    pub size: i64,
}
