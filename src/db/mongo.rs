use crate::models::Subscriber;
use actix_web::web;
use futures::stream::TryStreamExt;
use mongodb::{options::ClientOptions, Client, Database};

pub async fn insert_subscriber(
    db: web::Data<Database>,
    subscriber: Subscriber,
) -> Result<(), mongodb::error::Error> {
    // Insert the subscriber into the database
    let collection = db.collection::<Subscriber>("subscribers");
    collection.insert_one(subscriber, None).await?;
    Ok(())
}

pub async fn get_subscribers(
    db: web::Data<Database>,
) -> Result<Vec<Subscriber>, mongodb::error::Error> {
    // Retrieve all subscribers from the database
    let collection = db.collection::<Subscriber>("subscribers");
    let mut cursor = collection.find(None, None).await?;
    let mut subscribers: Vec<Subscriber> = Vec::new();

    while let Some(result) = cursor.try_next().await? {
        subscribers.push(result);
    }

    Ok(subscribers)
}
#[allow(dead_code)]
pub async fn connect_db() -> Result<Database, mongodb::error::Error> {
    // Connect to MongoDB
    let mongodb_uri = std::env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let client_options = ClientOptions::parse(mongodb_uri).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("newsletter");
    println!("Connected to database");
    Ok(db)
}
