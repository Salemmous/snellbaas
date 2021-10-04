use auth::{models::users::User, services::AuthenticationService};
use error::{SBError, SBResult};
use mongodb::{bson::Document, options::FindOptions, Client};

#[derive(Clone)]
pub struct ProjectAuthService {
    client: Client,
    secret: String,
}

impl ProjectAuthService {
    pub fn new(client: Client, secret: String) -> ProjectAuthService {
        ProjectAuthService {
            client: client,
            secret: secret,
        }
    }

    pub async fn get_users(
        &self,
        project_id: &str,
        filter: Option<Document>,
        options: Option<FindOptions>,
    ) -> SBResult<Vec<User>> {
        let database = self.client.database(&format!("project-{}", project_id));
        let service =
            AuthenticationService::init(database, String::from("_auth"), self.secret.clone());
        service
            .users
            .get_users(filter, options)
            .await
            .map_err(|_| SBError::InternalServiceError {
                service: String::from("project_auth"),
                message: String::from("Failure listing users."),
            })
    }
}
