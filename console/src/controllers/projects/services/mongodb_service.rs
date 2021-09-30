use crate::models::project::ProjectUser;
use crate::services::project_mongodb::ProjectMongoDBService;
use actix_web::{http, web, HttpResponse, Responder, Scope};
use error::SBError;
use mongodb::{bson::Document, options::FindOptions};
use serde::Deserialize;
use web::Json;

#[derive(Deserialize)]
struct ProjectInfo {
    pub project_id: String,
}

#[derive(Deserialize)]
struct ProjectCollectionInfo {
    pub project_id: String,
    pub collection_name: String,
}

#[derive(Deserialize)]
struct ProjectDocumentQuery {
    pub filter: Option<Document>,
    pub options: Option<FindOptions>,
}

pub fn get_service() -> Scope {
    let resource = web::scope("/mongodb");

    resource
        .route("/collections", web::get().to(get_collection))
        .route(
            "/collections/{collection_name}/documents",
            web::post().to(get_documents),
        )
}

async fn get_collection(
    service: web::Data<ProjectMongoDBService>,
    info: web::Path<ProjectInfo>,
    _authorized_user: Option<ProjectUser>,
) -> impl Responder {
    let result = service.get_collections_for_project(&info.project_id).await;
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

async fn get_documents(
    service: web::Data<ProjectMongoDBService>,
    info: web::Path<ProjectCollectionInfo>,
    query: Json<ProjectDocumentQuery>,
    authorized_user: ProjectUser,
) -> impl Responder {
    let result = service
        .get_documents_from_collection(
            &info.project_id,
            &info.collection_name,
            query.filter.clone(),
            query.options.clone(),
        )
        .await;
    println!("{:?}", authorized_user);
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
