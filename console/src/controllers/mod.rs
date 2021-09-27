use actix_web::{web, Scope};

mod auth;
mod console;
mod projects;

pub fn get_service() -> Scope {
    let resource = web::scope("/api");

    resource
        .service(auth::get_service())
        .service(console::get_service())
        .service(projects::get_service())
}
