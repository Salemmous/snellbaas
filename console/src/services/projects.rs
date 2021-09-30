use crate::models::project::Project;
use error::{SBError, SBResult};
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::results::InsertOneResult;
use mongodb::Collection;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Deserialize, Debug, Serialize)]
pub struct MarshalledInsertOne {
    pub _id: String,
}

#[derive(Clone)]
pub struct ProjectService {
    collection: Collection<Project>,
}

impl ProjectService {
    pub fn new(collection: Collection<Project>) -> ProjectService {
        ProjectService { collection }
    }

    pub async fn get_projects_for_user(&self, user_id: &str) -> SBResult<Vec<Project>> {
        let cursor = self
            .collection
            .find(
                doc! {
                    "users": user_id
                },
                None,
            )
            .await
            .map_err(|_| SBError::InternalServiceError {
                service: String::from("projects"),
                message: String::from("Failure listing projects."),
            })?;
        cursor
            .try_collect::<Vec<Project>>()
            .await
            .map_err(|_| SBError::InternalServiceError {
                service: String::from("projects"),
                message: String::from("Failure listing projects."),
            })
    }

    pub async fn get(&self, project_id: &str) -> SBResult<Project> {
        let project_oid =
            ObjectId::from_str(project_id).map_err(|_| SBError::InternalServiceError {
                service: String::from("projects"),
                message: String::from("Failure making oid object."),
            })?;
        let res = self
            .collection
            .find_one(doc! {"_id":project_oid}, None)
            .await
            .map_err(|_| SBError::InternalServiceError {
                service: String::from("projects"),
                message: String::from("Failure finding project."),
            })?;
        match res {
            Some(r) => Ok(r),
            None => Err(SBError::ServiceError {
                service: String::from("projects"),
                message: String::from("No project found"),
            }),
        }
    }

    pub async fn create(&self, project: Project) -> SBResult<MarshalledInsertOne> {
        self.collection
            .insert_one(project, None)
            .await
            .map(|r: InsertOneResult| MarshalledInsertOne {
                _id: r.inserted_id.as_object_id().unwrap().to_hex(),
            })
            .map_err(|_| SBError::InternalServiceError {
                service: String::from("projects"),
                message: String::from("Could not create user."),
            })
    }

    pub async fn get_user_access_to_project(
        &self,
        project_id: &str,
        user_id: &str,
    ) -> SBResult<Option<Project>> {
        let project_oid =
            ObjectId::from_str(project_id).map_err(|_| SBError::InternalServiceError {
                service: String::from("projects"),
                message: String::from("Failure making oid object."),
            })?;
        let res = self
            .collection
            .find_one(doc! {"_id":project_oid, "users": user_id}, None)
            .await
            .map_err(|_| SBError::InternalServiceError {
                service: String::from("projects"),
                message: String::from("Failure finding project."),
            })?;
        Ok(res)
    }
}
