use actix_web::web::Json;
use bson::Document;
use futures::TryStreamExt;
use mongodb::{Collection, Database};
use serde_json::json;

use crate::types;

pub struct Core {
    users: Collection<Document>,
    apps: Collection<Document>,
    reviews: Collection<Document>,
    personal_libraries: Collection<Document>,
}

impl Core {
    pub fn new(db: &Database) -> Core {
        Core {
            users: db.collection("users"),
            apps: db.collection("apps"),
            reviews: db.collection("reviews"),
            personal_libraries: db.collection("personal_libraries"),
        }
    }
    pub async fn get_apps(&self) -> Vec<Document> {
        self.get_collection(&self.apps).await
    }

    pub async fn signup(&self, user: &Json<types::User>) -> serde_json::Value {
        json! ({
            "token":"todo"
        })
    }

    async fn get_collection(&self, collection: &Collection<Document>) -> Vec<Document> {
        let cursor = match collection.find(None, None).await {
            Ok(cursor) => cursor,
            Err(_) => return vec![],
        };

        cursor.try_collect().await.unwrap_or_else(|_| vec![])
    }
}
