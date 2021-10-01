use error::{SBError, SBResult};
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::{
    bson::{doc, Document},
    options::{
        CreateCollectionOptions, DeleteOptions, DropCollectionOptions, FindOneOptions, FindOptions,
        InsertOneOptions, UpdateModifications, UpdateOptions,
    },
    results::{CollectionSpecification, DeleteResult, UpdateResult},
    Client,
};
use std::str::FromStr;

#[derive(Clone)]
pub struct ProjectMongoDBService {
    client: Client,
}

impl ProjectMongoDBService {
    pub fn new(client: Client) -> ProjectMongoDBService {
        ProjectMongoDBService { client: client }
    }

    pub async fn get_collections_for_project(
        &self,
        project_id: &str,
    ) -> SBResult<Vec<CollectionSpecification>> {
        let database = self.client.database(&format!("project-{}", project_id));
        let cursor = database
            .list_collections(doc! {"name": {"$regex": "^(?!_)"}}, None)
            .await
            .map_err(|_| SBError::InternalServiceError {
                service: String::from("mongodb"),
                message: String::from("Failure listing projects."),
            })?;
        cursor
            .try_collect::<Vec<CollectionSpecification>>()
            .await
            .map_err(|_| SBError::InternalServiceError {
                service: String::from("mongodb"),
                message: String::from("Failure listing projects."),
            })
    }

    pub async fn get_documents_from_collection(
        &self,
        project_id: &str,
        collection_name: &str,
        filter: Option<Document>,
        options: Option<FindOptions>,
    ) -> SBResult<Vec<Document>> {
        let database = self.client.database(&format!("project-{}", project_id));
        let cursor = database
            .collection(collection_name)
            .find(filter, options)
            .await
            .map_err(|_| SBError::InternalServiceError {
                service: String::from("mongodb"),
                message: String::from("Failure querying documents."),
            })?;
        cursor
            .try_collect::<Vec<Document>>()
            .await
            .map_err(|_| SBError::InternalServiceError {
                service: String::from("mongodb"),
                message: String::from("Failure querying documents."),
            })
    }

    pub async fn get_document_by_id_from_collection(
        &self,
        project_id: &str,
        collection_name: &str,
        document_id: &str,
        options: Option<FindOneOptions>,
    ) -> SBResult<Option<Document>> {
        let oid = ObjectId::from_str(document_id).map_err(|_| SBError::InternalServiceError {
            service: String::from("mongodb"),
            message: String::from("Failure making oid object."),
        })?;
        let database = self.client.database(&format!("project-{}", project_id));
        database
            .collection(collection_name)
            .find_one(doc! {"_id": oid}, options)
            .await
            .map_err(|_| SBError::InternalServiceError {
                service: String::from("mongodb"),
                message: String::from("Failure querying documents."),
            })
    }

    pub async fn create_document(
        &self,
        project_id: &str,
        collection_name: &str,
        document: Document,
        options: Option<InsertOneOptions>,
    ) -> SBResult<Document> {
        let database = self.client.database(&format!("project-{}", project_id));
        database
            .collection(collection_name)
            .insert_one(document, options)
            .await
            .map(|r| {
                doc! {
                    "_id": r.inserted_id.as_object_id().unwrap().to_hex(),
                }
            })
            .map_err(|_| SBError::InternalServiceError {
                service: String::from("mongodb"),
                message: String::from("Failure creating document."),
            })
    }

    pub async fn create_collection(
        &self,
        project_id: &str,
        collection_name: &str,
        options: Option<CreateCollectionOptions>,
    ) -> SBResult<()> {
        let database = self.client.database(&format!("project-{}", project_id));
        database
            .create_collection(collection_name, options)
            .await
            .map_err(|_| SBError::InternalServiceError {
                service: String::from("mongodb"),
                message: String::from("Failure creating collection."),
            })
    }

    pub async fn drop_collection(
        &self,
        project_id: &str,
        collection_name: &str,
        options: Option<DropCollectionOptions>,
    ) -> SBResult<()> {
        let database = self.client.database(&format!("project-{}", project_id));
        database
            .collection::<Document>(collection_name)
            .drop(options)
            .await
            .map_err(|_| SBError::InternalServiceError {
                service: String::from("mongodb"),
                message: String::from("Failure creating collection."),
            })
    }

    pub async fn delete_documents(
        &self,
        project_id: &str,
        collection_name: &str,
        filter: Document,
        options: Option<DeleteOptions>,
    ) -> SBResult<DeleteResult> {
        let database = self.client.database(&format!("project-{}", project_id));
        database
            .collection::<Document>(collection_name)
            .delete_many(filter, options)
            .await
            .map_err(|_| SBError::InternalServiceError {
                service: String::from("mongodb"),
                message: String::from("Failure deleting document."),
            })
    }

    pub async fn delete_document(
        &self,
        project_id: &str,
        collection_name: &str,
        document_id: &str,
        options: Option<DeleteOptions>,
    ) -> SBResult<DeleteResult> {
        let oid = ObjectId::from_str(document_id).map_err(|_| SBError::InternalServiceError {
            service: String::from("mongodb"),
            message: String::from("Failure making oid object."),
        })?;
        self.delete_documents(project_id, collection_name, doc! {"_id": oid}, options)
            .await
    }

    pub async fn update_documents(
        &self,
        project_id: &str,
        collection_name: &str,
        filter: Document,
        update: Document,
        options: Option<UpdateOptions>,
    ) -> SBResult<UpdateResult> {
        let database = self.client.database(&format!("project-{}", project_id));
        database
            .collection::<Document>(collection_name)
            .update_many(
                filter,
                UpdateModifications::Document(doc! {"$set": update}),
                options,
            )
            .await
            .map_err(|_| SBError::InternalServiceError {
                service: String::from("mongodb"),
                message: String::from("Failure updating document."),
            })
    }

    pub async fn update_document(
        &self,
        project_id: &str,
        collection_name: &str,
        document_id: &str,
        update: Document,
        options: Option<UpdateOptions>,
    ) -> SBResult<UpdateResult> {
        let oid = ObjectId::from_str(document_id).map_err(|_| SBError::InternalServiceError {
            service: String::from("mongodb"),
            message: String::from("Failure making oid object."),
        })?;
        self.update_documents(
            project_id,
            collection_name,
            doc! {"_id": oid},
            update,
            options,
        )
        .await
    }
}
