extern crate core;

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
use actix_web::{App, HttpServer, web};
use env_logger::Env;
use utoipa::OpenApi;
use utoipa_actix_web::{AppExt, scope};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

#[derive(Clone)]
struct AppState {
    service_name: String,
    vehicle_service: VehicleService,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let database = init_mongodb().await.database("vehicle");
    let state = AppState {
        service_name: String::from("vehicle"),
        vehicle_service: VehicleService::new(database.clone()),
    };
    let base_route = format!("/{}", &state.service_name);

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
            .service(scope(&*base_route).configure(init_v1))
            .openapi_service(|api_doc| SwaggerUi::new("/docs/{_:.*}").url("/openapi.json", api_doc))
            .into_app()
    })
    .bind(("127.0.0.1", 8080))?
    .workers(4)
    .run()
    .await
}
