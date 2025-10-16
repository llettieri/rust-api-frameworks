mod vehicles;

use crate::routers::vehicles::init_vehicles;
use utoipa_actix_web::scope;
use utoipa_actix_web::service_config::ServiceConfig;

pub fn init_v1(config: &mut ServiceConfig, service_name: &str) {
    let base_route = format!("/{}/v1", service_name);
    config.service(scope(base_route.as_str()).configure(init_vehicles));
}
