use crate::app::dto::{CmdToListQuery, RepoInfoListDTO};
use crate::{
    app::dto::{CmdToRepoInfo, RepoInfoDTO},
    domain::repo_info::RepoImpl,
    utils::error::Result,
};
use async_trait::async_trait;

#[derive(Debug)]
pub struct RepoService<T>
where
    T: RepoImpl,
{
    s: Box<T>,
}

#[async_trait]
pub trait RepoServiceImpl: Send + Sync {
    async fn repo_info(&self, id: String) -> Result<RepoInfoDTO>;
    async fn add(&self, v: CmdToRepoInfo) -> Result<()>;
    async fn list(&self, v: CmdToListQuery) -> Result<RepoInfoListDTO>;
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
