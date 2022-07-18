use actix_web::{get, post, web, HttpResponse, Responder};

#[post("/api/signup")]
pub async fn signup(
    app_data: web::Data<crate::AppState>,
    user: web::Json<crate::types::User>,
) -> impl Responder {
    HttpResponse::Ok().json(app_data.core.signup(&user).await)
}

#[get("/api/apps")]
pub async fn apps(app_data: web::Data<crate::AppState>) -> impl Responder {
    HttpResponse::Ok().json(app_data.core.get_apps().await)
}
