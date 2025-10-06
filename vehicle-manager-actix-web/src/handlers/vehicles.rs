use crate::schemas::vehicle::VehicleSchema;

use actix_web::error::ErrorNotFound;
use actix_web::web::Json;
use actix_web::Error;
use paperclip::actix::{
    api_v2_operation,
    web::{self},
};

pub fn init_vehicles(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/vehicles")
            .service(web::resource("").route(web::get().to(get_vehicles)))
            .service(web::resource("/{vehicle_id}").route(web::get().to(get_vehicle))),
    );
}
#[api_v2_operation]
async fn get_vehicles() -> Result<Json<Vec<VehicleSchema>>, Error> {
    Ok(Json(vec![]))
}

#[api_v2_operation]
async fn get_vehicle(vehicle_id: web::Path<String>) -> Result<Json<VehicleSchema>, Error> {
    if vehicle_id.len() < 2 {
        return Err(ErrorNotFound("E_VEHICLE_NOT_FOUND"));
    }

    Ok(Json(VehicleSchema {
        id: vehicle_id.into_inner(),
        brand: String::from("Audi"),
    }))
}
