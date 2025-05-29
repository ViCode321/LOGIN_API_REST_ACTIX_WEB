use crate::interfaces::controllers::users::access_controller as Users_Endpoints;

use utoipa::{
	openapi::{
		self,
		security::{Http, HttpAuthScheme, SecurityScheme},
	},
	Modify, OpenApi,
};

#[derive(OpenApi)]
#[openapi(
	paths(
		Users_Endpoints::register,
		Users_Endpoints::get_users,
		Users_Endpoints::get_user_by_id,
		Users_Endpoints::login
	),
	tags((name = "API", description = "Basic API")),
	modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;
impl Modify for SecurityAddon {
	fn modify(&self, openapi: &mut openapi::OpenApi) {
		// NOTE: we can unwrap safely since there already is components registered.
		let components = openapi.components.as_mut().unwrap();
		components.add_security_scheme(
			"Token",
			SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
		);
	}
}
