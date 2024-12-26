pub mod user;

use actix_web::web;

pub fn define_routes(cfg: &mut web::ServiceConfig) {
    user::registration::define_routes(cfg);
}
