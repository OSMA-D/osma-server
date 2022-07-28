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
    apps_versions: Collection<Document>,
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
            apps_versions: db.collection("apps_versions"),
            reviews: db.collection("reviews"),
            personal_libraries: db.collection("personal_libraries"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET not found"),
            salt: env::var("SALT").expect("Hash salt not found"),
        }
    }
    pub async fn get_apps(&self) -> Vec<Document> {
        self.get_collection(&self.apps).await
    }

    pub async fn get_app(&self, name: &String) -> serde_json::Value {
        let response = self.apps.find_one(doc! {"app_id":&name}, None).await;
        match response {
            Ok(response) => match response {
                Some(result) => json!({
                    "code":"ok_body",
                    "body": result
                }),
                None => {
                    json! ({
                        "code":"denied",
                        "msg":"This app does not exist"
                    })
                }
            },
            Err(_) => {
                json! ({
                    "code":"err",
                    "msg":"Unknown error"
                })
            }
        }
    }

    pub async fn get_rating(&self, app_id: &String) -> serde_json::Value {
        let result = self
            .reviews
            .aggregate(
                [
                    doc! {"$match":{"app_id":app_id}},
                    doc! {
                        "$group": {
                            "_id": "$app_id",
                            "rating": {
                                "$avg": "$score"
                            },
                        },
                    },
                ],
                None,
            )
            .await;

        match result {
            Ok(mut result) => match result.next().await {
                Some(result) => match result {
                    Ok(result) => {
                        json! ({
                            "code":"ok_body",
                            "body":result
                        })
                    }
                    Err(_) => {
                        json! ({
                            "code":"err",
                            "msg":"Unknown error"
                        })
                    }
                },
                None => {
                    json! ({
                        "code":"denied",
                        "msg":"This app does not exist"
                    })
                }
            },
            Err(_) => {
                json! ({
                    "code":"err",
                    "msg":"Unknown error"
                })
            }
        }
    }

    async fn create_personal_library(&self, name: &String) {
        self.personal_libraries
            .insert_one(
                doc! {
                    "name":name,
                    "apps":[]
                },
                None,
            )
            .await
            .unwrap();
    }

    pub async fn get_reviews(&self, app_id: &String) -> Vec<Document> {
        self.get_collection_with_params(&self.reviews, doc! {"app_id":app_id})
            .await
    }

    pub async fn get_versions(&self, app_id: &String) -> Vec<Document> {
        self.get_collection_with_params_and_sort(
            &self.apps_versions,
            doc! {"app_id":app_id},
            doc! {"timestamp": -1},
        )
        .await
    }

    pub async fn write_review(&self, name: &String, info: &Json<ReviewData>) -> serde_json::Value {
        let options = FindOneOptions::builder()
            .projection(doc! {"_id" : 1})
            .build();
        let response = self
            .apps
            .find_one(doc! {"app_id":&info.app_id}, options)
            .await;

        match response {
            Ok(response) => match response {
                Some(_) => {
                    let options = UpdateOptions::builder().upsert(Some(true)).build();
                    let response = self
                        .reviews
                        .update_one(
                            doc! {"user_name": name},
                            doc! {"$set": {
                                "app_id":&info.app_id,
                                "text":&info.text,
                                "score":&info.score,
                                "timestamp":Utc::now().timestamp()
                            }},
                            options,
                        )
                        .await;
                    match response {
                        Ok(_) => {
                            json! ({
                                "code":"ok",
                                "msg":"The review is written"
                            })
                        }
                        Err(_) => {
                            json! ({
                                "code":"err",
                                "msg":"Validation error"
                            })
                        }
                    }
                }
                None => {
                    json! ({
                        "code":"denied",
                        "msg":"This app does not exist"
                    })
                }
            },
            Err(_) => {
                json! ({
                    "code":"err",
                    "msg":"Unknown error"
                })
            }
        }
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

    pub async fn delete_app_from_personal_library(
        &self,
        name: &String,
        app: &String,
    ) -> serde_json::Value {
        let response = self
            .personal_libraries
            .update_one(
                doc! {"name": name},
                doc! {"$pull": {
                    "apps":&app,
                }},
                None,
            )
            .await;
        match response {
            Ok(_) => {
                json! ({
                    "code":"ok",
                    "msg":"App deleted from personal library"
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

    pub async fn add_app_to_personal_library(
        &self,
        name: &String,
        app: &String,
    ) -> serde_json::Value {
        let options = FindOneOptions::builder()
            .projection(doc! {"_id" : 1})
            .build();
        let response = self.apps.find_one(doc! {"app_id":&app}, options).await;
        match response {
            Ok(response) => match response {
                Some(_) => {
                    let response = self
                        .personal_libraries
                        .update_one(
                            doc! {"name": name},
                            doc! {"$push": {
                                "apps":&app,
                            }},
                            None,
                        )
                        .await;
                    match response {
                        Ok(_) => {
                            json! ({
                                "code":"ok",
                                "msg":"App added to personal library"
                            })
                        }
                        Err(_) => {
                            json! ({
                                "code":"err",
                                "msg":"App already in the library | Unknown error"
                            })
                        }
                    }
                }
                None => {
                    json! ({
                        "code":"denied",
                        "msg":"This app does not exist"
                    })
                }
            },
            Err(_) => {
                json! ({
                    "code":"err",
                    "msg":"Unknown error"
                })
            }
        }
    }

    pub async fn change_password(
        &self,
        name: &String,
        old: &String,
        new: &String,
    ) -> serde_json::Value {
        let response = self.users.find_one(doc! {"name":&name}, None).await;
        match response {
            Ok(user) => match user {
                Some(user) => {
                    let old_pass_hash = self.hash(name.clone() + &old);
                    if &old_pass_hash == user.get_str("password").unwrap() {
                        let response = self
                            .users
                            .update_one(
                                doc! {"name": name},
                                doc! {"$set": {
                                    "password":&self.hash(name.clone() + &new),
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
                                    "msg":"Some error"
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
            },
            Err(_e) => {
                json! ({
                    "code":"err",
                    "msg":"User does not exist"
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
                        self.create_personal_library(&user.name).await;
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

    async fn get_collection_with_params_and_sort(
        &self,
        collection: &Collection<Document>,
        params: Document,
        sort_params: Document,
    ) -> Vec<Document> {
        let options = FindOptions::builder()
            .projection(doc! {"_id" : 0})
            .sort(sort_params)
            .build();
        let cursor = match collection.find(params, options).await {
            Ok(cursor) => cursor,
            Err(_) => return vec![],
        };

        cursor.try_collect().await.unwrap_or_else(|_| vec![])
    }

    async fn get_collection_with_params(
        &self,
        collection: &Collection<Document>,
        params: Document,
    ) -> Vec<Document> {
        let options = FindOptions::builder().projection(doc! {"_id" : 0}).build();
        let cursor = match collection.find(params, options).await {
            Ok(cursor) => cursor,
            Err(_) => return vec![],
        };

        cursor.try_collect().await.unwrap_or_else(|_| vec![])
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
