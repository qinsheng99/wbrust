use crate::utils::error::{ErrorMsg, Result};
use postgres::{Client, NoTls};
use std::{error::Error, result::Result as libResult};

#[allow(dead_code)]
fn pg_connect() -> Result<Client> {
    if let Ok(url) = std::env::var("PG_URL") {
        let conn = Client::connect(url.as_str(), NoTls).unwrap();

        return Ok(conn);
    }

    Err(ErrorMsg::new("failed connect"))
}

#[allow(dead_code)]
pub fn init() -> libResult<Client, Box<dyn Error>> {
    let cli = pg_connect()?;

    Ok(cli)
}
