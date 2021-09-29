use crate::models::users::{Claims, UpdateUser, User};
use bcrypt::{hash, verify};
use chrono::prelude::*;
use error::SBError;
use futures::TryStreamExt;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, document::Document};
use mongodb::options::{FindOptions, UpdateModifications};
use mongodb::results::InsertOneResult;
use mongodb::Collection;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

type UserServiceResult<T> = std::result::Result<T, SBError>;

#[derive(Deserialize, Debug, Serialize)]
pub struct MarshalledInsertOne {
    pub _id: String,
}

#[derive(Clone)]
pub struct UserService {
    collection: Collection<User>,
}

impl UserService {
    pub fn new(collection: Collection<User>) -> UserService {
        UserService { collection }
    }

    pub fn hash_password(&self, password: &str) -> UserServiceResult<String> {
        let cost = 4;
        let hashed = hash(password, cost).map_err(|_| SBError::InternalServiceError {
            service: String::from("users"),
            message: String::from("Failure hashing password"),
        })?;
        Ok(hashed)
    }

    pub async fn create(&self, user: User) -> UserServiceResult<MarshalledInsertOne> {
        if user.password == None {
            return Err(SBError::ServiceError {
                service: String::from("users"),
                message: String::from("No password provided"),
            });
        }

        let to_insert = user.clone();

        let check_username = self
            .collection
            .find_one(doc! {"username":user.username}, None)
            .await
            .map_err(|_| SBError::ServiceError {
                service: String::from("users"),
                message: String::from("No user found"),
            })?;

        let check_email = self
            .collection
            .find_one(doc! {"email":user.email}, None)
            .await
            .map_err(|_| SBError::ServiceError {
                service: String::from("users"),
                message: String::from("No user found"),
            })?;

        if !check_email.is_none() {
            return Err(SBError::ServiceError {
                service: String::from("users"),
                message: String::from("Username already in use."),
            });
        }

        if !check_username.is_none() {
            return Err(SBError::ServiceError {
                service: String::from("users"),
                message: String::from("Username already in use."),
            });
        }

        let hashed = self.hash_password(user.password.unwrap().as_ref())?;
        self.collection
            .insert_one(to_insert.copy_with_hash(hashed), None)
            .await
            .map(|r: InsertOneResult| MarshalledInsertOne {
                _id: r.inserted_id.as_object_id().unwrap().to_hex(),
            })
            .map_err(|_| SBError::InternalServiceError {
                service: String::from("users"),
                message: String::from("Could not create user."),
            })
    }

    pub async fn get(&self, user_id: &str) -> UserServiceResult<User> {
        let user_oid = ObjectId::from_str(user_id).map_err(|_| SBError::InternalServiceError {
            service: String::from("users"),
            message: String::from("Failure making oid object."),
        })?;
        let res = self
            .collection
            .find_one(doc! {"_id":user_oid}, None)
            .await
            .map_err(|_| SBError::InternalServiceError {
                service: String::from("users"),
                message: String::from("Failure finding user."),
            })?;
        match res {
            Some(r) => Ok(r.copy_without_hash()),
            None => Err(SBError::ServiceError {
                service: String::from("users"),
                message: String::from("No user found"),
            }),
        }
    }

    pub async fn update(&self, user_id: &str, updates: UpdateUser) -> UserServiceResult<Document> {
        let user_oid = ObjectId::from_str(user_id).map_err(|_| SBError::InternalServiceError {
            service: String::from("users"),
            message: String::from("Failure making oid object."),
        })?;

        let mut updates_doc = Document::new();

        if updates.email.is_some() {
            updates_doc.insert("email", updates.email.unwrap());
        }

        if updates.username.is_some() {
            updates_doc.insert("username", updates.username.unwrap());
        }

        let mods = UpdateModifications::Document(doc! {"$set":updates_doc});

        let result = self
            .collection
            .find_one_and_update(doc! {"_id":user_oid}, mods, None)
            .await;

        match result {
            Ok(_) => Ok(doc! { "success":true}),
            _ => Err(SBError::ServiceError {
                service: String::from("users"),
                message: String::from("User not found."),
            }),
        }
    }

    pub async fn delete(&self, user_id: &str) -> UserServiceResult<Document> {
        let user_oid = ObjectId::from_str(user_id).map_err(|_| SBError::InternalServiceError {
            service: String::from("users"),
            message: String::from("Failure making oid object."),
        })?;

        self.collection
            .delete_one(doc! {"_id":user_oid}, None)
            .await
            .map_err(|_| SBError::ServiceError {
                service: String::from("users"),
                message: String::from("Failure finding user."),
            })
            .map(|_| doc! {"success":true})
    }

    pub async fn authenticate(
        &self,
        email: &str,
        password: &str,
        secret: String,
    ) -> UserServiceResult<Document> {
        let res = self.collection.find_one(doc! { "email":email}, None).await;
        match res.clone() {
            Err(e) => println!("{}", e),
            _ => (),
        };
        if res.is_err() {
            return Err(SBError::InternalServiceError {
                service: String::from("users"),
                message: String::from("Could not query db."),
            });
        }
        let user_opt = res.unwrap();

        if user_opt.is_none() {
            return Err(SBError::ServiceError {
                service: String::from("users"),
                message: String::from("User not found."),
            });
        }

        let user = user_opt.unwrap();
        let hash = user.password;
        if hash == None {
            return Err(SBError::ServiceError {
                service: String::from("users"),
                message: String::from("Passwordless user."),
            });
        }
        let password_ok = verify(password, hash.unwrap().as_ref()).unwrap();

        match password_ok {
            true => {
                let my_claims = Claims {
                    sub: user.id.unwrap().to_hex(),
                    exp: Utc::now().timestamp() as usize + 172800,
                };
                let h = Header::new(Algorithm::HS256);
                let key = EncodingKey::from_secret(secret.as_ref());

                let token = encode(&h, &my_claims, &key).unwrap();
                let d = doc! {"token":token, "success":true};
                Ok(d)
            }
            _ => Err(SBError::ServiceError {
                service: String::from("users"),
                message: String::from("Authentication failed."),
            }),
        }
    }

    pub async fn get_users(&self, skip: u64, limit: i64) -> UserServiceResult<Vec<User>> {
        let find_options = FindOptions::builder().limit(limit).skip(skip).build();
        let cursor = self
            .collection
            .find(None, find_options)
            .await
            .map_err(|_| SBError::InternalServiceError {
                service: String::from("users"),
                message: String::from("Failure listing users."),
            })?;
        cursor
            .try_collect::<Vec<User>>()
            .await
            .map_err(|_| SBError::InternalServiceError {
                service: String::from("users"),
                message: String::from("Failure listing users."),
            })
    }
}
