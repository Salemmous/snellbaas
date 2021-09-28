use mongodb::Database;

pub mod users;

#[derive(Clone)]
pub struct AuthenticationService {
    pub db: Database,
    pub secret: String,
    pub users: users::UserService,
}

impl AuthenticationService {
    pub fn init(db: Database, collection_name: String, secret: String) -> AuthenticationService {
        let collection = db.collection(collection_name.as_ref());
        AuthenticationService {
            db: db,
            secret: secret,
            users: users::UserService::new(collection),
        }
    }
}
