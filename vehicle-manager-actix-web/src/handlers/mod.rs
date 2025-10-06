pub mod vehicles;

use crate::handlers::vehicles::init_vehicles;

use paperclip::actix::web;

pub fn init_root(config: &mut web::ServiceConfig, base_route: &String) {
    config.service(web::scope(&base_route).service(web::scope("/v1").configure(init_vehicles)));
}
