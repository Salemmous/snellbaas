use crate::services::projects::ProjectService;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::{dev, web, Error, FromRequest, HttpRequest};
use auth::models::users::Claims;
use auth::services::AuthenticationService;
use futures::Future;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use validator::Validate;

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<mongodb::bson::oid::ObjectId>,
    pub name: String,
    #[serde(default)]
    pub users: Vec<String>,
}

#[derive(Deserialize, Debug, Serialize, Validate)]
pub struct ProjectUser {
    pub token: String,
    pub sub: String,
    pub project: Project,
}

impl FromRequest for ProjectUser {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        let req_clone = req.clone();
        Box::pin(async move {
            let project_id = match req_clone.match_info().get("project_id") {
                Some(id) => id,
                None => return Err(ErrorInternalServerError("No project id given for guard")),
            };
            match req_clone.headers().get("Authorization") {
                Some(val) => match val.to_str() {
                    Ok(v) => {
                        let my_slice: Vec<&str> = v.split(" ").collect();
                        let auth_service =
                            match req_clone.app_data::<web::Data<AuthenticationService>>() {
                                Some(service) => service,
                                None => {
                                    return Err(ErrorInternalServerError(
                                        "Authentication not configured",
                                    ))
                                }
                            };
                        let key = DecodingKey::from_secret(auth_service.secret.as_ref());

                        if my_slice.len() != 2 {
                            return Err(ErrorBadRequest("Bad Headers"));
                        }

                        let token_claims_res =
                            decode::<Claims>(my_slice[1], &key, &Validation::new(Algorithm::HS256));

                        if token_claims_res.is_err() {
                            return Err(ErrorBadRequest("Not Authorized"));
                        }

                        let claims = token_claims_res.unwrap().claims;
                        let project_service =
                            match req_clone.app_data::<web::Data<ProjectService>>() {
                                Some(service) => service,
                                None => {
                                    return Err(ErrorInternalServerError(
                                        "Project service not configured",
                                    ))
                                }
                            };

                        let authorized_user = match project_service
                            .get_user_access_to_project(project_id, &claims.sub)
                            .await
                        {
                            Ok(Some(project)) => ProjectUser {
                                token: v.into(),
                                sub: claims.sub,
                                project: project,
                            },
                            _ => return Err(ErrorInternalServerError("No project access")),
                        };
                        Ok(authorized_user)
                    }
                    Err(e) => Err(ErrorBadRequest(e)),
                },
                None => Err(ErrorBadRequest("Not Authorized")),
            }
        })
    }
}
