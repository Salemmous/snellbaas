use actix_web::{web, Scope};

mod read;
mod write;

pub fn get_service() -> Scope {
    let resource = web::scope("/projects");

    resource
        .service(write::get_service())
        .service(read::get_service())
}
