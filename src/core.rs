use actix_web::web::Json;

use bson::{doc, Document};
use chrono::Utc;
use futures::{StreamExt, TryStreamExt};
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::{
    options::{FindOneOptions, FindOptions, UpdateOptions},
    Collection, Database,
};
use serde_json::json;
use sha3::{Digest, Sha3_256};
use std::env;

use crate::types::*;
pub struct Core {
    users: Collection<Document>,
    apps: Collection<Document>,
    reviews: Collection<Document>,
    personal_libraries: Collection<Document>,
    jwt_secret: String,
    salt: String,
}

impl Core {
    pub fn new(db: &Database) -> Core {
        Core {
            users: db.collection("users"),
            apps: db.collection("apps"),
            reviews: db.collection("reviews"),
            personal_libraries: db.collection("personal_libraries"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET not found"),
            salt: env::var("SALT").expect("Hash salt not found"),
        }
    }
    pub async fn get_apps(&self) -> Vec<Document> {
        self.get_collection(&self.apps).await
    }

    pub async fn update_user(&self, name: &String, info: &Json<UserData>) -> serde_json::Value {
        let response = self
            .users
            .update_one(
                doc! {"name": name},
                doc! {"$set": {
                    "email":&info.email,
                    "img":&info.img
                }},
                None,
            )
            .await;
        match response {
            Ok(_) => {
                json! ({
                    "code":"ok",
                    "msg":"User information updated"
                })
            }
            Err(_) => {
                json! ({
                    "code":"err",
                    "msg":"Unknown error"
                })
            }
        }
    }
    pub async fn signin(&self, name: &String, password: &String) -> serde_json::Value {
        let response = self.users.find_one(doc! {"name":name}, None).await;
        match response {
            Ok(user) => {
                match user {
                    Some(user) => {
                        let pass_hash = self.hash(name.clone() + &password);
                        if user.get_str("password").unwrap() == pass_hash {
                            let jwt_info = JwtInfo {
                                name: name.clone(),
                                role: user.get_str("role").unwrap().to_string(),
                                exp: Utc::now().timestamp() + 604800, //week
                            };

                            let token = encode(
                                &Header::default(),
                                &jwt_info,
                                &EncodingKey::from_secret(self.jwt_secret.as_ref()),
                            );

                            match token {
                                Ok(token) => {
                                    json! ({
                                        "code":"ok",
                                        "token":token
                                    })
                                }
                                Err(_) => {
                                    json! ({
                                        "code":"err",
                                        "msg":"Some problem with jwt generation"
                                    })
                                }
                            }
                        } else {
                            json! ({
                                "code":"denied",
                                "msg":"Wrong password"
                            })
                        }
                    }
                    None => {
                        json! ({
                            "code":"err",
                            "msg":"User does not exist"
                        })
                    }
                }
            }
            Err(_) => {
                json! ({
                    "code":"err",
                    "msg":"User does not exist"
                })
            }
        }
    }

    pub async fn signup(&self, user: &Json<User>) -> serde_json::Value {
        let jwt_info = JwtInfo {
            name: user.name.clone(),
            role: "user".to_string(),
            exp: Utc::now().timestamp() + 604800, //week
        };

        let token = encode(
            &Header::default(),
            &jwt_info,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        );

        match token {
            Ok(token) => {
                let auth_info = doc! {
                    "name": &user.name,
                    "password": self.hash(user.name.clone() + &user.password),
                    "email": &user.email,
                    "role": "user".to_string(),
                };
                let response = self.users.insert_one(&auth_info, None).await;
                match response {
                    Ok(_) => {
                        json! ({
                            "code":"ok",
                            "token":token
                        })
                    }
                    Err(_) => {
                        json! ({
                            "code":"err",
                            "msg":"User with this name already exist"
                        })
                    }
                }
            }
            Err(_) => {
                json! ({
                    "code":"err",
                    "msg":"Some problem with jwt generation"
                })
            }
        }
    }
    fn hash(&self, to_hash: String) -> String {
        let mut hasher = Sha3_256::new();
        hasher.update(to_hash + &self.salt);
        let hash = hasher.finalize();
        format!("{:x}", hash)
    }
    async fn get_collection(&self, collection: &Collection<Document>) -> Vec<Document> {
        let options = FindOptions::builder().projection(doc! {"_id" : 0}).build();
        let cursor = match collection.find(None, options).await {
            Ok(cursor) => cursor,
            Err(_) => return vec![],
        };

        cursor.try_collect().await.unwrap_or_else(|_| vec![])
    }
}
