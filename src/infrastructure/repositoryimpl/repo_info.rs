use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait};
use sqlx::types::uuid;

use crate::{
    app::dto::{CmdToListQuery, CmdToRepoInfo},
    common::infrastructure::postgresql::PgDB,
    domain::repo_info::{ListRepoInfo, RepoImpl, RepoInfo},
    infrastructure::repositoryimpl::repo_info_do::{RepoInfoDO, to_repo_info_do},
    utils::error::Result,
};
use crate::domain::repo_info::NewRepoInfoImpl;
use crate::infrastructure::repositoryimpl::dto;
use crate::infrastructure::repositoryimpl::repo_info_do::Total;
use crate::utils::error::Error;

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

#[derive(Clone)]
#[allow(dead_code)]
pub struct NewRepoInfo<'a> {
    new_db: &'a DatabaseConnection,
}
#[allow(dead_code)]
impl<'a> NewRepoInfo<'a> {
    pub fn new(new_db: &'a DatabaseConnection) -> Self {
        Self { new_db }
    }
}

#[async_trait]
impl RepoImpl for RepoInfoImpl {
    async fn repo_detail_info(&self, id: String) -> Result<RepoInfo> {
        let v: RepoInfoDO =
            sqlx::query_as(&*format!("SELECT * FROM {} WHERE uuid = $1", self.table))
                .bind(uuid::Uuid::parse_str(&id)?)
                .fetch_one(&self.db)
                .await?;

        Ok(RepoInfo::from(v))
    }

    async fn add(&self, v: CmdToRepoInfo) -> Result<()> {
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

    async fn total(&self, _v: CmdToListQuery) -> Result<i64> {
        let v: Option<Total> =
            sqlx::query_as(&*format!("SELECT COUNT(uuid) as total FROM {}", self.table))
                .fetch_optional(&self.db)
                .await?;

        if let Some(total) = v {
            return Ok(total.total);
        }

        Ok(0)
    }

    async fn list(&self, v: CmdToListQuery) -> Result<ListRepoInfo> {
        let total = self.total(v.clone()).await?;
        let mut l = ListRepoInfo::default();
        if total == 0 {
            return Ok(l);
        }

        let v: Vec<RepoInfoDO> =
            sqlx::query_as(&*format!("SELECT * FROM {} LIMIT $1 OFFSET $2", self.table))
                .bind(&v.size)
                .bind((v.page - 1) * v.size)
                .fetch_all(&self.db)
                .await?;

        let mut info_list: Vec<RepoInfo> = vec![];
        for item in v {
            info_list.push(RepoInfo::from(item))
        }

        l.repo_list = info_list;
        l.total = total;
        Ok(l)
    }
}
#[async_trait]
#[allow(dead_code)]
impl<'a> NewRepoInfoImpl for NewRepoInfo<'a> {
    async fn repo_detail_info_for_sea(&self, id: u64) -> Result<()> {
        match dto::Entity::find_by_id(id).one(self.new_db).await? {
            None => Err(Error::NotFound),
            Some(user) => {
                println!("{:?}", user);
                
                Ok(())
            }
        }
    }
}
