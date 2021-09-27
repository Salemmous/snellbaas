use crate::database;
use error::SBError;
use mongodb::Database;

pub mod users;

#[derive(Clone)]
pub struct AuthenticationService {
    pub db: Database,
    pub secret: String,
    pub users: users::UserService,
}

impl AuthenticationService {
    pub async fn init(
        db_url: String,
        db_name: String,
        collection_name: String,
        secret: String,
    ) -> std::result::Result<AuthenticationService, SBError> {
        let db = database::get_db(db_url, db_name).await?;
        let collection = db.collection(collection_name.as_ref());
        let service = AuthenticationService {
            db: db,
            secret: secret,
            users: users::UserService::new(collection),
        };
        Ok(service)
    }
}
