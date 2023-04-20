use crate::common::util::{ErrorMsg, Result};
use postgres::{Client, NoTls};

#[allow(dead_code)]
pub fn pg_connect() -> Result<Client> {
    if let Ok(url) = std::env::var("PG_URL") {
        let conn = Client::connect(url.as_str(), NoTls).unwrap();

        return Ok(conn);
    }

    Err(ErrorMsg::new("failed connect"))
}
