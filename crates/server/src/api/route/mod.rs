pub mod user;

use actix_web::web;

pub fn define_routes(cfg: &mut web::ServiceConfig) {
    user::register::define_routes(cfg);
}
