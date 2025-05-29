// src/interfaces/routes/user_routes.rs

use actix_web::web;
use crate::interfaces::controllers::users::access_controller::{
	register, get_users, get_user_by_id, login
};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
	cfg.service(
		web::scope("/api/v1")
			.service(register)
			.service(get_users)
			.service(get_user_by_id)
			.service(login)
	);
}
