// Copyright (c) 2023 artegoser (Artemy Egorov)
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

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
pub struct ReviewData {
    pub app_id: String,
    pub score: i32,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordsInf {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppInfo {
    pub app_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppTags {
    pub tags: Vec<String>,
}
