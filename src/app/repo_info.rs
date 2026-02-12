use async_trait::async_trait;

use crate::{
    app::dto::{CmdToListQuery, CmdToRepoInfo, RepoInfoDTO, RepoInfoListDTO, RepoInfoModelDTO},
    domain::repo_info::{NewRepoInfoImpl, RepoImpl},
    utils::error::Result,
};

#[derive(Debug)]
pub struct RepoService<T>
where
    T: RepoImpl,
{
    s: Box<T>,
}

#[derive(Debug)]
pub struct NewRepoService<T>
where
    T: NewRepoInfoImpl,
{
    s: Box<T>,
}

#[async_trait]
pub trait RepoServiceImpl: Send + Sync {
    async fn repo_info(&self, id: String) -> Result<RepoInfoDTO>;
    async fn add(&self, v: CmdToRepoInfo) -> Result<()>;
    async fn list(&self, v: CmdToListQuery) -> Result<RepoInfoListDTO>;
}

#[async_trait]
pub trait NewRepoServiceImpl: Send + Sync {
    async fn repo_info(&self, id: u64) -> Result<RepoInfoModelDTO>;
}

impl<T> RepoService<T>
where
    T: RepoImpl,
{
    #[allow(dead_code)]
    pub fn new(s: Box<T>) -> Self {
        RepoService { s }
    }
}

impl<T> NewRepoService<T>
where
    T: NewRepoInfoImpl + Clone + 'static,
{
    #[allow(dead_code)]
    pub fn new(s: Box<T>) -> Self {
        NewRepoService { s }
    }
}

#[async_trait]
impl<T> RepoServiceImpl for RepoService<T>
where
    T: RepoImpl + Sync,
{
    async fn repo_info(&self, id: String) -> Result<RepoInfoDTO> {
        let info = self.s.repo_detail_info(id).await?;

        RepoInfoDTO::from(info)
    }

    async fn add(&self, v: CmdToRepoInfo) -> Result<()> {
        Ok(self.s.add(v).await?)
    }

    async fn list(&self, v: CmdToListQuery) -> Result<RepoInfoListDTO> {
        let v = self.s.list(v).await?;

        RepoInfoListDTO::from(v)
    }
}

#[async_trait]
impl<T> NewRepoServiceImpl for NewRepoService<T>
where
    T: NewRepoInfoImpl + Sync,
{
    async fn repo_info(&self, id: u64) -> Result<RepoInfoModelDTO> {
        let data = self.s.repo_detail_info_for_sea(id).await?;

        RepoInfoModelDTO::from(data)
    }
}
