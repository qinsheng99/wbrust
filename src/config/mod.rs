use crate::utils::file::{read_file, read_file_to_yaml};
use serde::{Deserialize, Serialize};
use serde_with_expand_env::with_expand_envs;
use std::error::Error;

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

pub fn load_config(path: &str) -> Result<impl ConfigImpl, Box<dyn Error>> {
    let content = read_file(path).unwrap();

    match read_file_to_yaml::<Config>(content.as_str()) {
        Ok(cfg) => Ok(cfg),
        Err(e) => Err(e),
    }
}
