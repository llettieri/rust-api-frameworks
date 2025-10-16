mod config;
mod helpers;
mod models;
mod routers;
mod schemas;
mod services;

use crate::config::init_mongodb;
use crate::routers::init_v1;
use crate::services::vehicle::VehicleService;
use actix_web::http::StatusCode;
use actix_web::middleware::{ErrorHandlerResponse, ErrorHandlers, Logger};
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

const SERVICE_NAME: &str = "vehicle";

#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

#[derive(Clone)]
pub struct AppState {
    service_name: String,
    vehicle_service: VehicleService,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let database = init_mongodb(SERVICE_NAME).await;
    let state = AppState {
        service_name: String::from(SERVICE_NAME),
        vehicle_service: VehicleService::new(database.clone()),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(Logger::new("[%s] - %r - %a - %{User-Agent}i - [%Ts]"))
            .wrap(
                ErrorHandlers::new().handler(StatusCode::INTERNAL_SERVER_ERROR, |response| {
                    Ok(ErrorHandlerResponse::Response(
                        response.map_into_left_body(),
                    ))
                }),
            )
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .configure(|config| init_v1(config, &state.service_name))
            .openapi_service(|api_doc| SwaggerUi::new("/docs/{_:.*}").url("/openapi.json", api_doc))
            .into_app()
    })
    .bind(("127.0.0.1", 8080))?
    .workers(4)
    .run()
    .await
}
