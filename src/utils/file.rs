use serde::Deserialize;
use std::error::Error;
use std::fs;

#[allow(dead_code)]
pub(crate) fn read_file(path: &'static str) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;

    Ok(content)
}

#[allow(dead_code)]
pub fn read_file_to_yaml<'a, T: Deserialize<'a>>(content: &'a str) -> Result<(), Box<dyn Error>> {
    match serde_yaml::from_str::<'a, T>(content) {
        Ok(_s) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}
