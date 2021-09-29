use actix_web::{web, Scope};

mod mongodb_service;

pub fn get_service() -> Scope {
    let resource = web::scope("/services/{project_id}");

    resource.service(mongodb_service::get_service())
}
