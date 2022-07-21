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
pub struct UserAuth {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtInfo {
    pub name: String,
    pub role: String,
    pub exp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub email: String,
    pub img: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordsInf {
    pub old_password: String,
    pub new_password: String,
}
