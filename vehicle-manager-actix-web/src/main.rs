mod config;
mod handlers;
mod schemas;
mod models;

use crate::config::init_mongodb;
use crate::handlers::init_root;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use mongodb::Client;
use paperclip::actix::OpenApiExt;

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

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap_api()
            .with_json_spec_at(format!("/{}/docs", &state.service_name).as_str())
            .configure({
                let base_root = format!("/{}", &state.service_name);
                move |config| init_root(config, &base_root)
            })
            .wrap(Logger::new("[%s] - %r - %a - %{User-Agent}i - [%Ts]"))
            .build()
    })
    .bind(("127.0.0.1", 8080))?
    .workers(4)
    .run()
    .await
}
