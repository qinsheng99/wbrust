use crate::{
    common::infrastructure::postgresql::PgDB,
    domain::repo_info::{RepoImpl, RepoInfo},
    infrastructure::repositoryimpl::repo_info_do::RepoInfoDO,
};
use async_trait::async_trait;

#[derive(Debug)]
pub struct RepoInfoImpl {
    db: PgDB,
    table: String,
}

impl RepoInfoImpl {
    pub fn new(db: PgDB, table: String) -> Self {
        Self { db, table }
    }
}

#[async_trait]
impl RepoImpl for RepoInfoImpl {
    async fn repo_detail_info(&self, id: String) -> crate::utils::error::Result<RepoInfo> {
        let v: RepoInfoDO =
            sqlx::query_as(&*format!("SELECT * FROM {} WHERE uuid = ?", self.table))
                .bind(&id)
                .fetch_one(&self.db)
                .await?;

        Ok(RepoInfo::from(v))
    }
}
