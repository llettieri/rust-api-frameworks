mod config;
mod handlers;
mod models;
mod schemas;

use crate::config::init_mongodb;
use crate::handlers::init_v1;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use mongodb::Client;
use utoipa::OpenApi;
use utoipa_actix_web::{scope, AppExt};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

#[derive(Clone)]
struct AppState {
    service_name: String,
    db_client: Client,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let state = AppState {
        service_name: String::from("vehicle"),
        db_client: init_mongodb().await,
    };
    let base_route = format!("/{}", &state.service_name);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(Logger::new("[%s] - %r - %a - %{User-Agent}i - [%Ts]"))
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
