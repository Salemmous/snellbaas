use crate::models::project::Project;
use crate::services::projects::ProjectService;
use actix_web::{http, web, HttpResponse, Responder, Scope};
use auth::models::users::AuthorizedUser;
use error::SBError;
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    pub project_id: String,
}

pub fn get_service() -> Scope {
    let resource = web::scope("/info");

    resource
        .route("/list", web::get().to(get_projects_for_user))
        .route("/{project_id}", web::get().to(get_project))
}

async fn get_projects_for_user(
    service: web::Data<ProjectService>,
    authorized_user: AuthorizedUser,
) -> impl Responder {
    let result = service.get_projects_for_user(&authorized_user.sub).await;
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

async fn get_project(
    service: web::Data<ProjectService>,
    info: web::Path<Info>,
    authorized_user: AuthorizedUser,
) -> impl Responder {
    let result: std::result::Result<Project, SBError> = service.get(&info.project_id).await;
    match result {
        Ok(result) => {
            if !result.users.contains(&authorized_user.sub) {
                return HttpResponse::Unauthorized().finish();
            }
            HttpResponse::Ok().json(result)
        }
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
