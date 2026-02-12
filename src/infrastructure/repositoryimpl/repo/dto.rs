use {
    crate::domain::repo_info::RepoInfoModel,
    sea_orm::entity::prelude::*,
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "repo_info")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub owner: String,
    pub repo: String,
    pub branch: String,
    pub status: String,
    pub last_commit: String,
    pub timeout: i32,
    pub modified_time: u64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for RepoInfoModel {
    fn from(model: Model) -> RepoInfoModel {
        RepoInfoModel {
            id: model.id,
            owner: model.owner,
            repo: model.repo,
            branch: model.branch,
            status: model.status,
            last_commit: model.last_commit,
            timeout: model.timeout,
            modified_time: model.modified_time,
        }
    }
}
