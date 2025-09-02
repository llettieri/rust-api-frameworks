use crate::models::vehicle::Vehicle;
use crate::services::vehicles::get_vehicle;
use axum::extract::Path;
use axum::routing::{get, post};
use axum::{debug_handler, Json, Router};

pub fn vehicles_router() -> Router {
    Router::new()
        .route("/{id}", get(get_vehicle_details))
        .route("/", post(create_vehicle))
}
async fn get_vehicle_details(Path(vehicle_id): Path<String>) -> Json<Vehicle> {
    Json::from(get_vehicle(vehicle_id).await)
}

#[debug_handler]
async fn create_vehicle() {
    println!("Creating a new vehicle");
}
