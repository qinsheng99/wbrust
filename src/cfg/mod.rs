use {
    config::{Config as Cfg, File, FileFormat},
    serde::{Deserialize, Serialize},
    serde_with_expand_env::with_expand_envs,
    std::sync::{Arc, RwLock},
};

use crate::utils::file::{read_file, read_file_to_yaml};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Config {
    #[serde(deserialize_with = "with_expand_envs")]
    access_token: String,
}

#[allow(dead_code)]
pub trait ConfigImpl {
    fn access_token(&self) -> &String;
}

impl ConfigImpl for Config {
    fn access_token(&self) -> &String {
        &self.access_token
    }
}

#[allow(dead_code)]
pub fn load_config(path: &str) -> Arc<Config> {
    let content = read_file(path).expect("read file failed");

    Arc::new(read_file_to_yaml::<Config>(content.as_str()).expect("marshal yaml failed"))
}

pub struct LocalConfig {
    #[allow(dead_code)]
    path: String,
    pub config: Arc<RwLock<Cfg>>,
}

impl LocalConfig {
    pub fn new(path: &str) -> LocalConfig {
        let cfg = Cfg::builder()
            .add_source(File::new(path, FileFormat::Toml))
            .build()
            .expect("load file failed");

        LocalConfig {
            path: path.to_string(),
            config: Arc::new(RwLock::new(cfg)),
        }
    }
}
