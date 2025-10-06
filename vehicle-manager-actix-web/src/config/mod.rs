use mongodb::Client;

pub async fn init_mongodb() -> Client {
    log::info!("Initializing MongoDB...");

    Client::with_uri_str("mongodb://mongodb:27017")
        .await
        .expect("Failed to connect to the MongoDB database!")
}
