use actix_cors::Cors;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use auth::services::AuthenticationService;
use dotenv;
use error::SBError;
use std::env;
use std::result::Result;

mod controllers;
mod models;
mod services;

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn get_var(name: &str) -> String {
    env::var(name).expect(format!("Expected environment variable {} to be set", name).as_ref())
}

async fn build_auth_data() -> Result<AuthenticationService, SBError> {
    let db_url = get_var("MONGO_DB_URL");
    let db_name = get_var("MONGO_DB_NAME");
    let user_collection_name = get_var("MONGO_USER_COLLECTION");
    let secret = get_var("SECRET");

    AuthenticationService::init(db_url, db_name, user_collection_name, secret).await
}

fn build_project_data(
    authentication_service: AuthenticationService,
) -> services::projects::ProjectService {
    let project_collection_name = get_var("MONGO_PROJECT_COLLECTION");
    services::projects::ProjectService::new(
        authentication_service
            .db
            .collection(project_collection_name.as_ref()),
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let authentication_service = build_auth_data()
        .await
        .expect("Authentication service init failed");
    let project_service = build_project_data(authentication_service.clone());
    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(authentication_service.clone()))
            .app_data(web::Data::new(project_service.clone()))
            .service(hello)
            .service(controllers::get_service())
            .service(controllers::console::get_service())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    use actix_web::web::Bytes;

    #[actix_rt::test]
    async fn test_index_get() {
        let mut app = test::init_service(App::new().service(hello)).await;
        let req = test::TestRequest::get().to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
        let response_content = test::read_body(resp).await;
        assert_eq!(response_content, Bytes::from_static(b"Hello world!"));
    }
}
