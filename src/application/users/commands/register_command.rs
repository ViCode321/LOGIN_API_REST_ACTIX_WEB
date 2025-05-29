use sea_orm::entity::prelude::*;

pub struct RegisterCommand {
	pub encrypted_password: String,
	pub email: String,
	pub primer_apellido: String,
	pub segundo_nombre: Option<String>,
	pub primer_nombre: String,
}
