use mongodb::Client;

pub async fn init_mongodb() -> Client {
    log::info!("Initializing MongoDB...");

    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .expect("Failed to connect to the MongoDB database!");

    log::info!("Connected to MongoDB");

    client
}
