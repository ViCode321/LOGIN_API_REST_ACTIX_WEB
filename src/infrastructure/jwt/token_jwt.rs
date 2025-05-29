// infrastructure/jwt/token_jwt.rs
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use crate::config::env::JWT_SECRET;

#[derive(Serialize, Deserialize)]
struct Claims {
	sub: i64,
	exp: usize,
}

pub fn generate_jwt(user_id: i64) -> Result<String, String> {
	let expiration = Utc::now() + Duration::hours(24);
	let claims = Claims {
		sub: user_id,
		exp: expiration.timestamp() as usize,
	};

	encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET.as_bytes()))
		.map_err(|e| e.to_string())
}
