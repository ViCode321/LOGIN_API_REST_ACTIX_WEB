// src/lib.rs
pub mod application;
pub mod config;
pub mod domain;
pub mod infrastructure;
pub mod interfaces;

use actix_web::{web};
use crate::interfaces::routes::routes::all_routes;

pub fn app(cfg: &mut web::ServiceConfig) {
	cfg.configure(all_routes);
}
