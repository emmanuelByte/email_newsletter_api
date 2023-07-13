// use crate::models::subscriber::Subscriber;
use crate::{
    db::{get_subscribers, insert_subscriber},
    models::Subscriber,
};
use actix_web::{get, post, web, HttpResponse, Responder};
use mongodb::Database;
use validator::Validate;

#[post("/subscribe")]
pub async fn subscribe(
    db: web::Data<Database>,
    subscriber: web::Json<Subscriber>,
) -> impl Responder {
    let subscriber = subscriber.into_inner();
    // Insert the subscriber into the database

    match subscriber.validate() {
        Ok(_) => {}
        Err(e) => {
            return HttpResponse::BadRequest().json(e);
        }
    }
    match insert_subscriber(db, subscriber).await {
        Ok(_) => HttpResponse::Ok().body("Subscribed successfully"),
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::InternalServerError().body("Failed to subscribe");
        }
    }
}
#[get("/subscribers")]
pub async fn get_all_subscribers(db: web::Data<Database>) -> impl Responder {
    // Retrieve all subscribers from the database
    match get_subscribers(db).await {
        Ok(subscribers) => HttpResponse::Ok().json(subscribers),
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve subscribers"),
    }
}

#[allow(dead_code)]
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(subscribe).service(get_all_subscribers);
}
