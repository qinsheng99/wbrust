use crate::app::dto::CmdToRepoInfo;
use crate::{
    common::infrastructure::postgresql::PgDB,
    domain::repo_info::{RepoImpl, RepoInfo},
    infrastructure::repositoryimpl::repo_info_do::{to_repo_info_do, RepoInfoDO},
};
use async_trait::async_trait;
use sqlx::types::uuid;

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
            sqlx::query_as(&*format!("SELECT * FROM {} WHERE uuid = $1", self.table))
                .bind(uuid::Uuid::parse_str(&id)?)
                .fetch_one(&self.db)
                .await?;

        Ok(RepoInfo::from(v))
    }

    async fn add(&self, v: CmdToRepoInfo) -> crate::utils::error::Result<()> {
        let repo = to_repo_info_do(v).await?;

        let _: Option<RepoInfoDO> = sqlx::query_as(&*format!(
            "INSERT INTO {} \
            (uuid,owner,repo,branch,status,last_commit,timeout,modified_time) \
            VALUES \
            ($1,$2,$3,$4,$5,$6,$7,$8)",
            self.table
        ))
        .bind(&repo.uuid)
        .bind(&repo.owner)
        .bind(&repo.repo)
        .bind(&repo.branch)
        .bind(&repo.status)
        .bind(&repo.last_commit)
        .bind(repo.timeout)
        .bind(repo.modified_time)
        .fetch_optional(&self.db)
        .await?;

        Ok(())
    }

    // async fn
}
