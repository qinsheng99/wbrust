use crate::utils::file::{read_file, read_file_to_yaml};
use config::{Config as libConfig, File, FileFormat};
use serde::{Deserialize, Serialize};
use serde_with_expand_env::with_expand_envs;
use std::error::Error;
use std::sync::{Arc, RwLock};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Config {
    #[serde(deserialize_with = "with_expand_envs")]
    access_token: String,
}

pub trait ConfigImpl {
    fn access_token(&self) -> &String;
}

impl ConfigImpl for Config {
    fn access_token(&self) -> &String {
        &self.access_token
    }
}

pub fn load_config(path: &str) -> Arc<Config> {
    let content = read_file(path).expect("read file failed");

    Arc::new(read_file_to_yaml::<Config>(content.as_str()).expect("marshal yaml failed"))
}

pub struct LocalConfig {
    path: String,
    pub config: Arc<RwLock<libConfig>>,
}

impl LocalConfig {
    pub fn new(path: &str) -> LocalConfig {
        let cfg = libConfig::builder()
            .add_source(File::new(path, FileFormat::Toml))
            .build()
            .expect("load file failed");

        LocalConfig {
            path: path.to_string(),
            config: Arc::new(RwLock::new(cfg)),
        }
    }
}
