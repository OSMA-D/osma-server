use crate::types::*;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use actix_web_grants::proc_macro::{has_any_permission, has_permissions};
#[post("/signup")]
pub async fn signup(app_data: web::Data<crate::AppState>, user: web::Json<User>) -> impl Responder {
    response(app_data.core.signup(&user).await)
}
#[get("/apps")]
pub async fn apps(app_data: web::Data<crate::AppState>) -> impl Responder {
    HttpResponse::Ok().json(app_data.core.get_apps().await)
}
fn response(result: serde_json::Value) -> impl Responder {
    if result["code"] == "ok" {
        HttpResponse::Ok().json(result)
    } else if result["code"] == "ok_body" {
        HttpResponse::Ok().json(&result["body"])
    } else if result["code"] == "denied" {
        HttpResponse::Forbidden().json(result)
    } else {
        println!("{}", result["code"]);
        HttpResponse::InternalServerError().json(result)
    }
}
