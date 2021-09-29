use error::{SBError, SBResult};
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, Document},
    options::FindOptions,
    results::CollectionSpecification,
    Client,
};

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
}
