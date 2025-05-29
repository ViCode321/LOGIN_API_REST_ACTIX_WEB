// src/interfaces/routes/routes.rs

use actix_web::web;
use crate::interfaces::routes::user_routes;

pub fn all_routes(cfg: &mut web::ServiceConfig) {
	cfg.configure(user_routes::user_routes);
}