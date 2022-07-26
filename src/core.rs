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

    async fn get_collection(&self, collection: &Collection<Document>) -> Vec<Document> {
        let options = FindOptions::builder().projection(doc! {"_id" : 0}).build();
        let cursor = match collection.find(None, options).await {
            Ok(cursor) => cursor,
            Err(_) => return vec![],
        };

        cursor.try_collect().await.unwrap_or_else(|_| vec![])
    }
}
