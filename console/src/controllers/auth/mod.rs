use actix_web::{web, Scope};

mod login;
mod profile;
mod registration;
mod users;

pub fn get_service() -> Scope {
    let resource = web::scope("/auth");

    resource
        .service(users::get_service())
        .service(profile::get_service())
        .service(registration::get_service())
        .service(login::get_service())
}
