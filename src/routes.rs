use crate::types::*;
use actix_web::{get, post, web, HttpResponse, Responder};

#[post("/api/signup")]
pub async fn signup(app_data: web::Data<crate::AppState>, user: web::Json<User>) -> impl Responder {
    let result = app_data.core.signup(&user).await;

    if result["code"] != "ok" {
        HttpResponse::Forbidden().json(result)
    } else {
        HttpResponse::Ok().json(result)
    }
}

#[post("/api/signin")]
pub async fn signin(
    app_data: web::Data<crate::AppState>,
    user: web::Json<UserAuth>,
) -> impl Responder {
    let result = app_data.core.signin(&user.name, &user.password).await;
    if result["code"] == "ok" {
        HttpResponse::Ok().json(result)
    } else {
        HttpResponse::Forbidden().json(result)
    }
}

#[get("/api/apps")]
pub async fn apps(app_data: web::Data<crate::AppState>) -> impl Responder {
    HttpResponse::Ok().json(app_data.core.get_apps().await)
}
