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

#[get("/api/apps")]
pub async fn apps(app_data: web::Data<crate::AppState>) -> impl Responder {
    HttpResponse::Ok().json(app_data.core.get_apps().await)
}
