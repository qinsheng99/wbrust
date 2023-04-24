use crate::utils::error::{Error as localError, Result};
use postgres::{Client, NoTls};

#[allow(dead_code)]
fn pg_connect() -> Result<Client> {
    if let Ok(url) = std::env::var("PG_URL") {
        let conn = Client::connect(url.as_str(), NoTls).unwrap();

        return Ok(conn);
    }

    Err(localError::DataBaseError(String::from("no connect url")))
}

#[allow(dead_code)]
pub fn init() -> Result<Client> {
    let cli = pg_connect()?;

    Ok(cli)
}
