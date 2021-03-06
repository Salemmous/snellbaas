use auth::models::users::{AuthorizedUser, Info, UpdateUser};
use auth::services::AuthenticationService;

use actix_web::{http, web, HttpResponse, Resource, Responder};
use error::SBError;
use web::Json;

pub fn get_service() -> Resource {
    let resource = web::resource("/users/{user_id}");

    resource
        .route(web::get().to(get_single_user))
        .route(web::put().to(update_user))
        .route(web::delete().to(delete_single_user))
}

async fn update_user(
    service: web::Data<AuthenticationService>,
    info: web::Path<Info>,
    updates: Json<UpdateUser>,
    authorized_user: AuthorizedUser,
) -> impl Responder {
    if &authorized_user.sub != &info.user_id {
        return HttpResponse::Unauthorized().finish();
    }

    let result = service
        .users
        .update(&info.user_id, updates.into_inner())
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

async fn get_single_user(
    service: web::Data<AuthenticationService>,
    info: web::Path<Info>,
    authorized_user: AuthorizedUser,
) -> impl Responder {
    if &authorized_user.sub != &info.user_id {
        return HttpResponse::Unauthorized().finish();
    }

    let auth_res = service.users.get(&info.user_id).await;

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

async fn delete_single_user(
    service: web::Data<AuthenticationService>,
    info: web::Path<Info>,
    authorized_user: AuthorizedUser,
) -> impl Responder {
    if &authorized_user.sub != &info.user_id {
        return HttpResponse::Unauthorized().finish();
    }

    let delete_res = service.users.delete(&info.user_id).await;

    match delete_res {
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
