use crate::models::project::ProjectUser;
use crate::services::project_mongodb::ProjectMongoDBService;
use actix_web::{http, web, HttpResponse, Responder, Scope};
use error::SBError;
use mongodb::{
    bson::Document,
    options::{
        CreateCollectionOptions, DeleteOptions, DropCollectionOptions, FindOneOptions, FindOptions,
        InsertOneOptions, UpdateOptions,
    },
};
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
struct ProjectDocumentInfo {
    pub project_id: String,
    pub collection_name: String,
    pub document_id: String,
}

#[derive(Deserialize)]
struct ProjectCreateCollectionQuery {
    pub options: Option<CreateCollectionOptions>,
}

#[derive(Deserialize)]
struct ProjectDropCollectionQuery {
    pub options: Option<DropCollectionOptions>,
}

#[derive(Deserialize)]
struct ProjectDocumentQuery {
    pub filter: Option<Document>,
    pub options: Option<FindOptions>,
}

#[derive(Deserialize)]
struct ProjectCreateDocumentQuery {
    pub document: Document,
    pub options: Option<InsertOneOptions>,
}

#[derive(Deserialize)]
struct ProjectGetByIdDocumentQuery {
    pub options: Option<FindOneOptions>,
}

#[derive(Deserialize)]
struct ProjectDeleteDocumentByIdQuery {
    pub options: Option<DeleteOptions>,
}

#[derive(Deserialize)]
struct ProjectDeleteDocumentQuery {
    pub filter: Document,
    pub options: Option<DeleteOptions>,
}

#[derive(Deserialize)]
struct ProjectUpdateDocumentByIdQuery {
    pub update: Document,
    pub options: Option<UpdateOptions>,
}

#[derive(Deserialize)]
struct ProjectUpdateDocumentQuery {
    pub update: Document,
    pub filter: Document,
    pub options: Option<UpdateOptions>,
}

pub fn get_service() -> Scope {
    let resource = web::scope("/mongodb");

    resource
        .route("/collections", web::get().to(get_collection))
        .route(
            "/collections/{collection_name}/create",
            web::post().to(create_collection),
        )
        .route(
            "/collections/{collection_name}/drop",
            web::post().to(drop_collection),
        )
        .route(
            "/collections/{collection_name}/documents",
            web::post().to(get_documents),
        )
        .route(
            "/collections/{collection_name}/documents/delete",
            web::post().to(delete_documents),
        )
        .route(
            "/collections/{collection_name}/documents/update",
            web::post().to(update_documents),
        )
        .route(
            "/collections/{collection_name}/documents/{document_id}/get",
            web::post().to(get_document_by_id),
        )
        .route(
            "/collections/{collection_name}/documents/{document_id}/delete",
            web::post().to(delete_document_by_id),
        )
        .route(
            "/collections/{collection_name}/documents/{document_id}/update",
            web::post().to(update_document_by_id),
        )
        .route(
            "/collections/{collection_name}/documents/create",
            web::post().to(create_document),
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

async fn create_collection(
    service: web::Data<ProjectMongoDBService>,
    info: web::Path<ProjectCollectionInfo>,
    options: Json<ProjectCreateCollectionQuery>,
    _authorized_user: Option<ProjectUser>,
) -> impl Responder {
    let result = service
        .create_collection(
            &info.project_id,
            &info.collection_name,
            options.options.clone(),
        )
        .await;
    match result {
        Ok(_) => HttpResponse::Ok().json(true),
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

async fn drop_collection(
    service: web::Data<ProjectMongoDBService>,
    info: web::Path<ProjectCollectionInfo>,
    options: Json<ProjectDropCollectionQuery>,
    _authorized_user: Option<ProjectUser>,
) -> impl Responder {
    let result = service
        .drop_collection(
            &info.project_id,
            &info.collection_name,
            options.options.clone(),
        )
        .await;
    match result {
        Ok(_) => HttpResponse::Ok().json(true),
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
    _authorized_user: ProjectUser,
) -> impl Responder {
    let result = service
        .get_documents_from_collection(
            &info.project_id,
            &info.collection_name,
            query.filter.clone(),
            query.options.clone(),
        )
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

async fn get_document_by_id(
    service: web::Data<ProjectMongoDBService>,
    info: web::Path<ProjectDocumentInfo>,
    options: Json<ProjectGetByIdDocumentQuery>,
    _authorized_user: ProjectUser,
) -> impl Responder {
    let result = service
        .get_document_by_id_from_collection(
            &info.project_id,
            &info.collection_name,
            &info.document_id,
            options.options.clone(),
        )
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

async fn create_document(
    service: web::Data<ProjectMongoDBService>,
    info: web::Path<ProjectCollectionInfo>,
    query: Json<ProjectCreateDocumentQuery>,
    _authorized_user: ProjectUser,
) -> impl Responder {
    let result = service
        .create_document(
            &info.project_id,
            &info.collection_name,
            query.document.clone(),
            query.options.clone(),
        )
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

async fn delete_documents(
    service: web::Data<ProjectMongoDBService>,
    info: web::Path<ProjectCollectionInfo>,
    query: Json<ProjectDeleteDocumentQuery>,
    _authorized_user: ProjectUser,
) -> impl Responder {
    let result = service
        .delete_documents(
            &info.project_id,
            &info.collection_name,
            query.filter.clone(),
            query.options.clone(),
        )
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

async fn delete_document_by_id(
    service: web::Data<ProjectMongoDBService>,
    info: web::Path<ProjectDocumentInfo>,
    query: Json<ProjectDeleteDocumentByIdQuery>,
    _authorized_user: ProjectUser,
) -> impl Responder {
    let result = service
        .delete_document(
            &info.project_id,
            &info.collection_name,
            &info.document_id,
            query.options.clone(),
        )
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

async fn update_documents(
    service: web::Data<ProjectMongoDBService>,
    info: web::Path<ProjectCollectionInfo>,
    query: Json<ProjectUpdateDocumentQuery>,
    _authorized_user: ProjectUser,
) -> impl Responder {
    let result = service
        .update_documents(
            &info.project_id,
            &info.collection_name,
            query.filter.clone(),
            query.update.clone(),
            query.options.clone(),
        )
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

async fn update_document_by_id(
    service: web::Data<ProjectMongoDBService>,
    info: web::Path<ProjectDocumentInfo>,
    query: Json<ProjectUpdateDocumentByIdQuery>,
    _authorized_user: ProjectUser,
) -> impl Responder {
    let result = service
        .update_document(
            &info.project_id,
            &info.collection_name,
            &info.document_id,
            query.update.clone(),
            query.options.clone(),
        )
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
