use error::SBError;
use mongodb::{Client, Database};

use std::result::Result;

pub async fn get_db_client(database_url: String) -> Result<Client, SBError> {
    let client = Client::with_uri_str(&database_url)
        .await
        .map_err(|_| SBError::DBConnectionError())?;
    Ok(client)
}
