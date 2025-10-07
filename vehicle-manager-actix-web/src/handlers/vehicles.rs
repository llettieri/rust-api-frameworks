use crate::schemas::vehicle::VehicleSchema;

use actix_web::error::ErrorNotFound;
use actix_web::web::Json;
use actix_web::{get, web, Error};
use utoipa_actix_web::scope;
use utoipa_actix_web::service_config::ServiceConfig;

pub fn init_vehicles(config: &mut ServiceConfig) {
    config.service(
        scope("/vehicles")
            .service(get_vehicles)
            .service(get_vehicle),
    );
}
#[utoipa::path(tag = "vehicle")]
#[get("")]
async fn get_vehicles() -> Result<Json<Vec<VehicleSchema>>, Error> {
    Ok(Json(vec![]))
}

#[utoipa::path(tag = "vehicle")]
#[get("/{vehicle_id}")]
async fn get_vehicle(vehicle_id: web::Path<String>) -> Result<Json<VehicleSchema>, Error> {
    if vehicle_id.len() < 2 {
        return Err(ErrorNotFound("E_VEHICLE_NOT_FOUND"));
    }

    Ok(Json(VehicleSchema {
        id: vehicle_id.into_inner(),
        brand: String::from("Audi"),
    }))
}
