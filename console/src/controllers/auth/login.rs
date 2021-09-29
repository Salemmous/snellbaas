use auth::models::users::AuthenticateUser;
use auth::services::AuthenticationService;

use actix_web::{http, web, HttpResponse, Resource, Responder};
use error::SBError;
use web::Json;

pub fn get_service() -> Resource {
    let resource = web::resource("/login");

    resource.route(web::post().to(authenticate_user))
}

async fn authenticate_user(
    service: web::Data<AuthenticationService>,
    user: Json<AuthenticateUser>,
) -> impl Responder {
    let private_key = (*service.secret).to_owned();
    let result = service
        .users
        .authenticate(&user.email, &user.password, private_key)
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
