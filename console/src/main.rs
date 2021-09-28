use actix_cors::Cors;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use auth::services::AuthenticationService;
use dotenv;
use error::SBError;
use std::env;
use std::result::Result;

mod controllers;
mod database;
mod models;
mod services;

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn get_var(name: &str) -> String {
    env::var(name).expect(format!("Expected environment variable {} to be set", name).as_ref())
}

async fn build_db_client_data() -> Result<mongodb::Client, SBError> {
    let db_url = get_var("MONGO_DB_URL");
    database::get_db_client(db_url)
        .await
        .map_err(|_| SBError::DBConnectionError())
}

fn build_db_data(client: mongodb::Client) -> mongodb::Database {
    let db_name = get_var("MONGO_DB_NAME");
    client.database(&db_name)
}

fn build_auth_data(db: mongodb::Database) -> AuthenticationService {
    let user_collection_name = get_var("MONGO_USER_COLLECTION");
    let secret = get_var("SECRET");

    AuthenticationService::init(db, user_collection_name, secret)
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
    let db_client = build_db_client_data().await.expect("DB Client init failed");
    HttpServer::new(move || {
        let cors = Cors::permissive();
        let db_client_data = db_client.clone();
        let db_data = build_db_data(db_client_data.clone());
        let authentication_service = build_auth_data(db_data.clone());
        let project_service = build_project_data(authentication_service.clone());
        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(db_client_data))
            .app_data(web::Data::new(db_data))
            .app_data(web::Data::new(authentication_service))
            .app_data(web::Data::new(project_service))
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
