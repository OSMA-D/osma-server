use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod core;
mod routes;
mod types;

pub struct AppState {
    core: core::Core,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port: u16 = env::var("PORT")
        .unwrap()
        .parse()
        .expect("PORT must be a number");

    let client_options =
        mongodb::options::ClientOptions::parse(env::var("MONGODB_URI").expect("Mongodb uri"))
            .await
            .unwrap();
    let client = mongodb::Client::with_options(client_options).unwrap();
    let db = client.database("osma");

    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin();
        App::new()
            .wrap(cors)
            .app_data(Data::new(AppState {
                core: core::Core::new(&db),
            }))
            .service(routes::apps)
            .service(routes::signup)
    })
    .bind(("0.0.0.0", port))
    .expect("Can not bind to port")
    .run()
    .await
}
