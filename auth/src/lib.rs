mod database;
pub mod models;
pub mod services;

#[cfg(test)]
mod tests {
    use super::*;
    use services::AuthenticationService;

    const DB_NAME: &str = "auth_test";
    const DB_URL: &str = "mongodb://localhost:27017";
    const COLLECTION_NAME: &str = "users";
    const SECRET: &str = "";

    #[tokio::test]
    async fn test_connection() {
        let service = AuthenticationService::init(
            DB_URL.to_owned(),
            DB_NAME.to_owned(),
            COLLECTION_NAME.to_owned(),
            SECRET.to_owned(),
        )
        .await;
        assert!(service.is_ok());
    }
}
