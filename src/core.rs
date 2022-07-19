use actix_web::web::Json;
use bson::{doc, Document};
use chrono::Utc;
use futures::TryStreamExt;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};
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
                    Ok(r) => {
                        json! ({
                            "code":"ok",
                            "token":token
                        })
                    }
                    Err(e) => {
                        json! ({
                            "code":"err",
                            "msg":"User with this name already exists"
                        })
                    }
                }
            }
            Err(e) => {
                json! ({
                    "code":"err",
                    "msg":"Some problem with jwt generation"
                })
            }
        }
    }
    fn hash(&self, toHash: String) -> String {
        let mut hasher = Sha3_256::new();
        hasher.update(toHash + &self.salt);
        let hash = hasher.finalize();
        format!("{:x}", hash)
    }
    async fn get_collection(&self, collection: &Collection<Document>) -> Vec<Document> {
        let cursor = match collection.find(None, None).await {
            Ok(cursor) => cursor,
            Err(_) => return vec![],
        };

        cursor.try_collect().await.unwrap_or_else(|_| vec![])
    }
}
