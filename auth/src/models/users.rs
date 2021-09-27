extern crate regex;

use crate::services::AuthenticationService;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::{dev, web, Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Deserialize, Debug, Serialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<mongodb::bson::oid::ObjectId>,
    #[validate(length(min = 3))]
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    pub first_name: String,
    pub last_name: String,
    #[validate(email)]
    pub email: String,
}

impl User {
    pub fn copy_with_hash(&self, hash: String) -> User {
        User {
            id: self.id,
            username: self.username.clone(),
            password: Some(hash),
            last_name: self.last_name.clone(),
            first_name: self.first_name.clone(),
            email: self.email.clone(),
        }
    }

    pub fn copy_without_hash(&self) -> User {
        User {
            id: self.id,
            username: self.username.clone(),
            password: None,
            last_name: self.last_name.clone(),
            first_name: self.first_name.clone(),
            email: self.email.clone(),
        }
    }
}

#[derive(Deserialize, Debug, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUser {
    #[validate(length(min = 3))]
    pub username: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Validate)]
pub struct AuthorizedUser {
    pub token: String,
    pub sub: String,
}

impl FromRequest for AuthorizedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        match req.headers().get("Authorization") {
            Some(val) => match val.to_str() {
                Ok(v) => {
                    let my_slice: Vec<&str> = v.split(" ").collect();
                    let service = match req.app_data::<web::Data<AuthenticationService>>() {
                        Some(service) => service,
                        None => {
                            return err(ErrorInternalServerError("Authentication not configured"))
                        }
                    };
                    let key = DecodingKey::from_secret(service.secret.as_ref());

                    if my_slice.len() != 2 {
                        return err(ErrorBadRequest("Bad Headers"));
                    }

                    let token_claims_res =
                        decode::<Claims>(my_slice[1], &key, &Validation::new(Algorithm::HS256));

                    if token_claims_res.is_err() {
                        return err(ErrorBadRequest("Not Authorized"));
                    }

                    let claims = token_claims_res.unwrap().claims;
                    let authorized_user = AuthorizedUser {
                        token: v.into(),
                        sub: claims.sub,
                    };
                    ok(authorized_user)
                }
                Err(e) => err(ErrorBadRequest(e)),
            },
            None => err(ErrorBadRequest("Not Authorized")),
        }
    }
}

#[derive(Deserialize, Debug, Serialize, Validate)]
pub struct AuthenticateUser {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct Info {
    pub user_id: String,
}

#[derive(Deserialize, Debug, Serialize, Validate)]
pub struct UserResponseData {
    _id: String,
}
