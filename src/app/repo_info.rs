use crate::{app::dto::RepoInfoDTO, domain::repo_info::RepoImpl, utils::error::Result};
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
}
