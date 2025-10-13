use mongodb::{Client, Database};

pub async fn init_mongodb(service_name: &str) -> Database {
    log::info!("Initializing MongoDB...");

    let client = Client::with_uri_str("mongodb://localhost:27017/vehicle")
        .await
        .expect("Failed to connect to the MongoDB database!");

    client.database(service_name)
}
