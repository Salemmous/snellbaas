use auth::models::users::AuthorizedUser;
use auth::services::AuthenticationService;

use actix_web::{http, web, HttpResponse, Resource, Responder};
use error::SBError;

pub fn get_service() -> Resource {
    let resource = web::resource("/profile");

    resource.route(web::get().to(get_single_user))
}

async fn get_single_user(
    service: web::Data<AuthenticationService>,
    authorized_user: AuthorizedUser,
) -> impl Responder {
    let auth_res = service.users.get(&authorized_user.sub).await;

    match auth_res {
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
