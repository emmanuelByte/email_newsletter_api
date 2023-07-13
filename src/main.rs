use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use std::env;
mod db;
mod handlers;
mod models;
use handlers::init_routes;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug");
    let port = env::var("PORT").expect("PORT must be set");
    let db = db::connect_db().await.unwrap();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(init_routes)
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
