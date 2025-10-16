use crate::schemas::vehicle::{CreateVehicleSchema, VehicleSchema};
use core::convert::Into;

use crate::helpers::pagination::{Page, Pagination};
use crate::schemas::errors::ApiError;
use crate::AppState;
use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use utoipa_actix_web::scope;
use utoipa_actix_web::service_config::ServiceConfig;

/// Initialize vehicle routes.
pub fn init_vehicles(config: &mut ServiceConfig) {
    config.service(
        scope("/vehicles")
            .service(get_vehicles)
            .service(create_vehicle)
            .service(get_vehicle)
            .service(update_vehicle)
            .service(delete_vehicle),
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

#[utoipa::path(
    tag = "vehicle",
    responses(
        (status = NO_CONTENT, description = "No Content"),
        (status = BAD_REQUEST, body = String, description = "Bad Request", example = "E_BAD_REQUEST"),
    ))
]
#[delete("/{vehicle_id}")]
async fn delete_vehicle(
    vehicle_id: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    state
        .vehicle_service
        .delete_vehicle(&vehicle_id)
        .await
        .map_err(|_| ApiError::InvalidObjectId)?;

    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use crate::config::init_mongodb;
    use crate::helpers::pagination::Page;
    use crate::routers::init_v1;
    use crate::schemas::vehicle::{CreateVehicleSchema, VehicleSchema};
    use crate::services::vehicle::VehicleService;
    use crate::{AppState, SERVICE_NAME};
    use actix_http::Request;
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::{test, web, App};
    use serde_json;
    use testcontainers::core::ContainerPort;
    use testcontainers::runners::AsyncRunner;
    use testcontainers::ImageExt;
    use testcontainers_modules::mongo::Mongo;
    use utoipa_actix_web::AppExt;

    async fn test_mongodb() {
        let container = Mongo::default()
            .with_mapped_port(27017, ContainerPort::Tcp(27017))
            .start()
            .await
            .unwrap();
    }

    pub async fn test_app()
    -> impl Service<Request, Response = ServiceResponse, Error = actix_web::Error> {
        test_mongodb().await;
        let database = init_mongodb(SERVICE_NAME).await;
        let state = AppState {
            service_name: String::from(SERVICE_NAME),
            vehicle_service: VehicleService::new(database.clone()),
        };

        test::init_service(
            App::new()
                .app_data(web::Data::new(state.clone()))
                .into_utoipa_app()
                .configure(|config| init_v1(config, &state.service_name))
                .into_app(),
        )
        .await
    }

    #[actix_web::test]
    async fn test_create_vehicle() {
        let app = test_app().await;

        let request = test::TestRequest::post()
            .uri("/vehicle/v1/vehicles")
            .set_json(CreateVehicleSchema {
                brand: "Toyota".to_string(),
                model: "Corolla".to_string(),
                ps: 120,
                mileage_in_km: 50000,
            })
            .to_request();
        let response = test::call_service(&app, request).await;

        assert!(response.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_vehicles() {
        let app = test_app().await;

        let request = test::TestRequest::get()
            .uri("/vehicle/v1/vehicles")
            .to_request();
        let response = test::call_service(&app, request).await;
        assert!(response.status().is_success());

        let body_bytes = test::read_body(response).await;
        let page: Page<VehicleSchema> = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(page.page, 1);
        assert_eq!(page.size, 50);
        assert_eq!(page.total, 1);
    }
}
