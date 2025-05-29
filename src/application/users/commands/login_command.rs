use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct LoginCommand {
	pub email: String,
	pub password: String,
}
