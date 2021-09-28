pub mod models;
pub mod services;

#[cfg(test)]
mod tests {
    use super::*;
    use error::SBError;
    use mongodb::{Client, Database};
    use services::AuthenticationService;

    const DB_NAME: &str = "auth_test";
    const DB_URL: &str = "mongodb://localhost:27017";
    const COLLECTION_NAME: &str = "users";
    const SECRET: &str = "";

    pub async fn get_db(database_url: String, database_name: String) -> Result<Database, SBError> {
        let client = Client::with_uri_str(&database_url)
            .await
            .map_err(|_| SBError::DBConnectionError())?;
        Ok(client.database(&database_name))
    }

    #[tokio::test]
    async fn test_connection() {
        let db = get_db(DB_NAME.to_owned(), DB_URL.to_owned()).await;
        assert!(db.is_ok());
        let service =
            AuthenticationService::init(db.unwrap(), COLLECTION_NAME.to_owned(), SECRET.to_owned());
    }
}
