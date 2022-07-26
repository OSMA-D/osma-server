use crate::types::*;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use actix_web_grants::proc_macro::{has_any_permission, has_permissions};

#[post("/signup")]
pub async fn signup(app_data: web::Data<crate::AppState>, user: web::Json<User>) -> impl Responder {
    response(app_data.core.signup(&user).await)
}

#[post("/signin")]
pub async fn signin(
    app_data: web::Data<crate::AppState>,
    user: web::Json<UserAuth>,
) -> impl Responder {
    response(app_data.core.signin(&user.name, &user.password).await)
}

#[get("/apps")]
#[has_any_permission("user", "admin")]
pub async fn apps(app_data: web::Data<crate::AppState>) -> impl Responder {
    HttpResponse::Ok().json(app_data.core.get_apps().await)
}

#[get("/reviews/{app_name_id}")]
#[has_any_permission("user", "admin")]
pub async fn reviews(
    app_data: web::Data<crate::AppState>,
    app_name_id: web::Path<String>,
) -> impl Responder {
    HttpResponse::Ok().json(app_data.core.get_reviews(&app_name_id).await)
}

#[post("/change_password")]
#[has_any_permission("user", "admin")]
pub async fn change_password(
    app_data: web::Data<crate::AppState>,
    info: web::Json<PasswordsInf>,
    req: HttpRequest,
) -> impl Responder {
    response(
        app_data
            .core
            .change_password(&username(req), &info.old_password, &info.new_password)
            .await,
    )
}

#[post("/update")]
#[has_any_permission("user", "admin")]
pub async fn update(
    app_data: web::Data<crate::AppState>,
    update_info: web::Json<UserData>,
    req: HttpRequest,
) -> impl Responder {
    response(
        app_data
            .core
            .update_user(&username(req), &update_info)
            .await,
    )
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
