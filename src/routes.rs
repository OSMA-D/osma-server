use crate::types::*;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use actix_web_grants::proc_macro::{has_any_permission, has_permissions};

#[post("/signup")]
pub async fn signup(app_data: web::Data<crate::AppState>, user: web::Json<User>) -> impl Responder {
    let result = app_data.core.signup(&user).await;

    if result["code"] != "ok" {
        HttpResponse::Forbidden().json(result)
    } else {
        HttpResponse::Ok().json(result)
    }
}

#[post("/signin")]
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

#[get("/apps")]
#[has_any_permission("user", "admin")]
pub async fn apps(app_data: web::Data<crate::AppState>) -> impl Responder {
    HttpResponse::Ok().json(app_data.core.get_apps().await)
}

#[post("/update")]
#[has_any_permission("user", "admin")]
pub async fn update(
    app_data: web::Data<crate::AppState>,
    update_info: web::Json<UserData>,
    req: HttpRequest,
) -> impl Responder {
    let result = app_data
        .core
        .update_user(&username(req), &update_info)
        .await;
    if result["code"] == "ok" {
        HttpResponse::Ok().json(result)
    } else {
        HttpResponse::Forbidden().json(result)
    }
}

fn username(req: HttpRequest) -> String {
    req.headers()
        .get("osma-username")
        .unwrap()
        .to_str()
        .ok()
        .unwrap()
        .to_string()
}

//req: HttpRequest
//secure = "username(req)==user.name"
