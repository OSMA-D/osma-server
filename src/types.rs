use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct App {
    id: String,
    name: String,
    description: String,
    version: String,
    platform: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthInfo {
    pub name: String,
    pub password: String,
    pub email: String,
    pub role: String,
}
