use crate::models::project::ProjectUser;
use crate::services::project_auth::ProjectAuthService;
use actix_web::{http, web, HttpResponse, Responder, Scope};
use error::SBError;
use mongodb::{bson::Document, options::FindOptions};
use serde::Deserialize;
use web::Json;

#[derive(Deserialize)]
struct ProjectInfo {
    pub project_id: String,
}

#[derive(Deserialize)]
struct ProjectAuthUserListQuery {
    pub filter: Option<Document>,
    pub options: Option<FindOptions>,
}

pub fn get_service() -> Scope {
    let resource = web::scope("/auth");
    resource.route("/users/get", web::post().to(get_users))
}

async fn get_users(
    service: web::Data<ProjectAuthService>,
    info: web::Path<ProjectInfo>,
    query: Json<ProjectAuthUserListQuery>,
    _authorized_user: ProjectUser,
) -> impl Responder {
    let result = service
        .get_users(
            &info.project_id,
            query.filter.clone(),
            query.options.clone(),
        )
        .await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(SBError::ServiceError {
            message,
            service: _,
        }) => HttpResponse::build(http::StatusCode::BAD_REQUEST).body(message),
        Err(error) => {
            println!("{}", error);
            HttpResponse::InternalServerError().finish()
        }
    }
}
