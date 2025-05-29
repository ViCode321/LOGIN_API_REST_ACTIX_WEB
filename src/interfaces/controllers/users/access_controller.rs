use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;
use crate::application::users::commands::{
	register_command::RegisterCommand,
	login_command::LoginCommand
};
use crate::application::users::handlers::{
	register_handler::RegisterHandler,
	get_user_handler::GetUserHandler,
	get_user_by_id_handler::GetUserByIdHandler,
	login_handler::LoginHandler,
};
use crate::infrastructure::{
	users::repository::UserRepositorySeaOrm,
	jwt::authenticate_jwt::AuthenticatedUser
};
use crate::config::db::connection;
use sea_orm::entity::prelude::*;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct RegisterRequest {
	pub encrypted_password: String,
	pub email: String,
	pub segundo_apellido: Option<String>,
	pub primer_apellido: String,
	pub segundo_nombre: Option<String>,
	pub primer_nombre: String,
}

#[utoipa::path(
	context_path = "/api/v1",
	path = "/register"
)]
#[post("/register")]
pub async fn register(body: web::Json<RegisterRequest>) -> impl Responder {
	let db = connection::establish_connection().await;

	let repo = UserRepositorySeaOrm { db };
	let handler = RegisterHandler { repo };

	let cmd = RegisterCommand {
		encrypted_password: body.encrypted_password.clone(),
		email: body.email.clone(),
		segundo_apellido: body.segundo_apellido.clone(),
		primer_apellido: body.primer_apellido.clone(),
		segundo_nombre: body.segundo_nombre.clone(),
		primer_nombre: body.primer_nombre.clone(),
	};

	match handler.handle(cmd).await {
		Ok(user) => HttpResponse::Ok().json(format!("Usuario {} registrado", user.email)),
		Err(err) => HttpResponse::BadRequest().body(err),
	}
}

#[utoipa::path(
	context_path = "/api/v1",
	path = "/get_users",
	security(
        ("Token" = [])
	)
)]
#[get("/get_users")]
pub async fn get_users(_auth: AuthenticatedUser) -> impl Responder {
	let db = connection::establish_connection().await;
	let repo = UserRepositorySeaOrm { db };
	let handler = GetUserHandler { repo };

	match handler.handle().await {
		Ok(users) => HttpResponse::Ok().json(users),
		Err(e) => HttpResponse::InternalServerError().body(e),
	}
}


#[utoipa::path(
	context_path = "/api/v1",
	path = "/get_user_by_id/{id}",
	security(
        ("Token" = [])
	)
)]
#[get("/get_user_by_id/{id}")]
pub async fn get_user_by_id(
	path: web::Path<i64>,
	_auth: AuthenticatedUser,
) -> impl Responder {
	let id = path.into_inner();

	let db = connection::establish_connection().await;
	let repo = UserRepositorySeaOrm { db };
	let handler = GetUserByIdHandler { repo };

	match handler.handle(id).await {
		Ok(user) => HttpResponse::Ok().json(user),
		Err(e) => HttpResponse::InternalServerError().body(e),
	}
}


#[utoipa::path(
	context_path = "/api/v1",
	path = "/login",
)]
#[post("/login")]
pub async fn login(body: web::Json<LoginCommand>) -> impl Responder {
	let db = connection::establish_connection().await;
	let repo = UserRepositorySeaOrm { db };
	let handler = LoginHandler { repo };

	match handler.handle(body.into_inner()).await {
		Ok(token) => HttpResponse::Ok().json(serde_json::json!({ "token": token })),
		Err(e) => HttpResponse::Unauthorized().body(e),
	}
}
