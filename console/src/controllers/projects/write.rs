use crate::models::project::Project;
use crate::services::projects::ProjectService;
use actix_web::{http, web, HttpResponse, Responder, Scope};
use auth::models::users::AuthorizedUser;
use error::SBError;
use web::Json;

pub fn get_service() -> Scope {
    let resource = web::scope("/edit");

    resource.route("/new", web::post().to(create_project))
}

async fn create_project(
    service: web::Data<ProjectService>,
    authorized_user: Option<AuthorizedUser>,
    project_payload: Json<Project>,
) -> impl Responder {
    let requestor = authorized_user.unwrap();

    let project = Project {
        id: Option::None,
        name: (*project_payload.name).to_owned(),
        users: vec![requestor.sub],
    };

    let result = service.create(project).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(SBError::UserServiceError { message }) => {
            HttpResponse::build(http::StatusCode::BAD_REQUEST).body(message)
        }
        Err(error) => {
            println!("{}", error);
            HttpResponse::InternalServerError().finish()
        }
    }
}
