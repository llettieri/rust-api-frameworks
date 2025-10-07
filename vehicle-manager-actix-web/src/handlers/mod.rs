pub mod vehicles;

use crate::handlers::vehicles::init_vehicles;
use utoipa_actix_web::scope;
use utoipa_actix_web::service_config::ServiceConfig;

pub fn init_v1(config: &mut ServiceConfig) {
    config.service(scope("/v1").configure(init_vehicles));
}
