use actix_cors::Cors;
use actix_web::{web, App, Error, HttpServer};
use dotenv::dotenv;
use std::env;

use actix_web::dev::ServiceRequest;
use actix_web::http::header::HeaderName;
use actix_web::http::header::HeaderValue;
use actix_web_grants::permissions::AttachPermissions;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use jsonwebtoken::{decode, DecodingKey, Validation};

mod core;
mod routes;
mod types;

pub struct AppState {
    core: core::Core,
}

async fn jwt_validator(
    mut req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = decode::<types::JwtInfo>(
        &credentials.token(),
        &DecodingKey::from_secret(
            env::var("JWT_SECRET")
                .expect("JWT_SECRET not found")
                .as_ref(),
        ),
        &Validation::default(),
    );
    match token {
        Ok(token) => {
            req.attach(vec![token.claims.role]);
            req.headers_mut().insert(
                HeaderName::from_lowercase(b"osma-username").unwrap(),
                HeaderValue::from_str(&token.claims.name).unwrap(),
            );

            Ok(req)
        }
        Err(_) => {
            req.attach(vec!["none".to_string()]);
            req.headers_mut().insert(
                HeaderName::from_lowercase(b"osma-username").unwrap(),
                HeaderValue::from_str("no").unwrap(),
            );
            Ok(req)
        }
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port: u16 = env::var("PORT")
        .unwrap()
        .parse()
        .expect("PORT must be a number");

    let client_options = mongodb::options::ClientOptions::parse(
        env::var("MONGODB_URI").expect("Mongodb uri not found"),
    )
    .await
    .unwrap();
    let client = mongodb::Client::with_options(client_options).unwrap();
    let db = client.database("osma");

    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin();
        App::new()
            .app_data(web::Data::new(AppState {
                core: core::Core::new(&db),
            }))
            .wrap(cors)
            .service(
                web::scope("/api")
                    .wrap(HttpAuthentication::bearer(jwt_validator))
                    .service(routes::apps)
                    .service(routes::app)
                    .service(routes::reviews)
                    .service(routes::rating)
                    .service(routes::versions)
                    .service(routes::update)
                    .service(routes::change_password)
                    .service(routes::write_review)
                    .service(routes::add_app_to_personal_library)
                    .service(routes::delete_app_from_personal_library),
            )
            .service(
                web::scope("/auth")
                    .service(routes::signup)
                    .service(routes::signin),
            )
    })
    .bind(("0.0.0.0", port))
    .expect("Can not bind to port")
    .run()
    .await
}
