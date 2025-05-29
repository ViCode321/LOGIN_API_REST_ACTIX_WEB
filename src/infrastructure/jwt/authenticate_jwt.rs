// infrastructure/jwt/authenticate_jwt.rs
use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
use futures_util::future::{ready, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use crate::config::env::JWT_SECRET;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
	pub sub: i64,
	pub exp: usize,
}

pub struct AuthenticatedUser(pub Claims);

impl FromRequest for AuthenticatedUser {
	type Error = Error;
	type Future = Ready<Result<Self, Self::Error>>;

	fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
		let token = req
			.headers()
			.get("Authorization")
			.and_then(|h| h.to_str().ok())
			.and_then(|auth| auth.strip_prefix("Bearer "))
			.map(str::to_string);

		if let Some(token) = token {
			//let key = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
			let result = decode::<Claims>(&token, &DecodingKey::from_secret(JWT_SECRET.as_bytes()), &Validation::default());

			match result {
				Ok(data) => ready(Ok(AuthenticatedUser(data.claims))),
				Err(_) => ready(Err(actix_web::error::ErrorUnauthorized("Invalid Token"))),
			}
		} else {
			ready(Err(actix_web::error::ErrorUnauthorized("Missing Token")))
		}
	}
}
