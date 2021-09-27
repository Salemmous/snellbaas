use error::SBError;
use mongodb::{Client, Database};

use std::result::Result;

pub async fn get_db(database_url: String, database_name: String) -> Result<Database, SBError> {
    let client = Client::with_uri_str(&database_url)
        .await
        .map_err(|_| SBError::DBConnectionError())?;
    Ok(client.database(&database_name))
}
