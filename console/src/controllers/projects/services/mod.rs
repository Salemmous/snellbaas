use actix_web::{web, Scope};

mod auth_service;
mod mongodb_service;

pub fn get_service() -> Scope {
    let resource = web::scope("/services/{project_id}");

    resource
        .service(mongodb_service::get_service())
        .service(auth_service::get_service())
}
