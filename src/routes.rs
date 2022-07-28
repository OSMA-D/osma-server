use crate::types::*;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use actix_web_grants::proc_macro::has_any_permission;

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

#[get("/reviews/{app_id}")]
#[has_any_permission("user", "admin")]
pub async fn reviews(
    app_data: web::Data<crate::AppState>,
    app_id: web::Path<String>,
) -> impl Responder {
    HttpResponse::Ok().json(app_data.core.get_reviews(&app_id).await)
}

#[get("/versions/{app_id}")]
#[has_any_permission("user", "admin")]
pub async fn versions(
    app_data: web::Data<crate::AppState>,
    app_id: web::Path<String>,
) -> impl Responder {
    HttpResponse::Ok().json(app_data.core.get_versions(&app_id).await)
}

#[get("/rating/{app_id}")]
#[has_any_permission("user", "admin")]
pub async fn rating(
    app_data: web::Data<crate::AppState>,
    app_id: web::Path<String>,
) -> impl Responder {
    response(app_data.core.get_rating(&app_id).await)
}

#[get("/app/{name}")]
#[has_any_permission("user", "admin")]
pub async fn app(app_data: web::Data<crate::AppState>, name: web::Path<String>) -> impl Responder {
    response(app_data.core.get_app(&name).await)
}

#[get("/latest_version/{name}")]
#[has_any_permission("user", "admin")]
pub async fn latest_version(
    app_data: web::Data<crate::AppState>,
    name: web::Path<String>,
) -> impl Responder {
    response(app_data.core.get_latest_version(&name).await)
}

#[get("/personal_library")]
#[has_any_permission("user", "admin")]
pub async fn personal_library(
    app_data: web::Data<crate::AppState>,
    req: HttpRequest,
) -> impl Responder {
    response(app_data.core.get_personal_library(&username(req)).await)
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

#[post("/add_app_to_personal_library")]
#[has_any_permission("user", "admin")]
pub async fn add_app_to_personal_library(
    app_data: web::Data<crate::AppState>,
    app_info: web::Json<AppInfo>,
    req: HttpRequest,
) -> impl Responder {
    response(
        app_data
            .core
            .add_app_to_personal_library(&username(req), &app_info.app_id)
            .await,
    )
}

#[post("/delete_app_from_personal_library")]
#[has_any_permission("user", "admin")]
pub async fn delete_app_from_personal_library(
    app_data: web::Data<crate::AppState>,
    app_info: web::Json<AppInfo>,
    req: HttpRequest,
) -> impl Responder {
    response(
        app_data
            .core
            .delete_app_from_personal_library(&username(req), &app_info.app_id)
            .await,
    )
}

#[post("/write_review")]
#[has_any_permission("user", "admin")]
pub async fn write_review(
    app_data: web::Data<crate::AppState>,
    review_data: web::Json<ReviewData>,
    req: HttpRequest,
) -> impl Responder {
    response(
        app_data
            .core
            .write_review(&username(req), &review_data)
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
        HttpResponse::InternalServerError().json(result)
    }
}

//req: HttpRequest
//secure = "username(req)==user.name"
