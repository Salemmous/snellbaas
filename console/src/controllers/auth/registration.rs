use auth::models::users::User;
use auth::services::AuthenticationService;

use actix_web::{http, web, HttpResponse, Resource, Responder};
use error::SBError;
use web::Json;

pub fn get_service() -> Resource {
    let resource = web::resource("/signup");

    resource.route(web::post().to(signup_user))
}

async fn signup_user(
    service: web::Data<AuthenticationService>,
    user: Json<User>,
) -> impl Responder {
    let result = service.users.create(user.into_inner()).await;
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
