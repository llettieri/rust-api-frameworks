use crate::schemas::vehicle::{CreateVehicleSchema, VehicleSchema};
use core::convert::Into;

use crate::AppState;
use crate::helpers::pagination::{Page, Pagination};
use crate::schemas::errors::ApiError;
use actix_web::{Error, HttpResponse, get, post, put, web};
use utoipa_actix_web::scope;
use utoipa_actix_web::service_config::ServiceConfig;

pub fn init_vehicles(config: &mut ServiceConfig) {
    config.service(
        scope("/vehicles")
            .service(get_vehicles)
            .service(create_vehicle)
            .service(get_vehicle)
            .service(update_vehicle),
    );
}
#[utoipa::path(
    tag = "vehicle",
    responses((status = OK, body = Page<VehicleSchema>, description = "Successful Response")),
    params(Pagination)
)]
#[get("")]
async fn get_vehicles(
    state: web::Data<AppState>,
    pagination: web::Query<Pagination>,
) -> Result<web::Json<Page<VehicleSchema>>, Error> {
    let page = state
        .vehicle_service
        .get_vehicles(pagination.page, pagination.size)
        .await;

    Ok(web::Json(page.map(Into::into)))
}

#[utoipa::path(tag = "vehicle", responses((status = CREATED, description = "Created")))]
#[post("")]
async fn create_vehicle(
    vehicle: web::Json<CreateVehicleSchema>,
    state: web::Data<AppState>,
) -> HttpResponse {
    state
        .vehicle_service
        .create_vehicle(vehicle.into_inner())
        .await;

    HttpResponse::Created().finish()
}

#[utoipa::path(
    tag = "vehicle",
    responses(
        (status = OK, body = VehicleSchema, description = "Successful Response"),
        (status = NOT_FOUND, body = String, description = "Not Found", example = "E_NOT_FOUND"),
        (status = BAD_REQUEST, body = String, description = "Bad Request", example = "E_BAD_REQUEST"),
    ))
]
#[get("/{vehicle_id}")]
async fn get_vehicle(
    vehicle_id: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<web::Json<VehicleSchema>, Error> {
    let vehicle = state
        .vehicle_service
        .get_vehicle_by_id(&vehicle_id)
        .await
        .map_err(|_| ApiError::InvalidObjectId)?;

    if vehicle.is_none() {
        return Err(Error::from(ApiError::NotFound));
    }

    Ok(web::Json(vehicle.unwrap().into()))
}

#[utoipa::path(tag = "vehicle", responses((status = OK, body = VehicleSchema, description = "Successful Response")))]
#[put("")]
async fn update_vehicle(
    vehicle: web::Json<VehicleSchema>,
    state: web::Data<AppState>,
) -> Result<web::Json<VehicleSchema>, Error> {
    let vehicle = state
        .vehicle_service
        .update_vehicle(vehicle.into_inner())
        .await;

    if vehicle.is_none() {
        return Err(Error::from(ApiError::NotFound));
    }

    Ok(web::Json(vehicle.unwrap().into()))
}
