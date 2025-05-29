use bcrypt::verify;

pub fn verify_password(hash: &str, password: &str) -> Result<bool, String> {
	verify(password, hash).map_err(|e| e.to_string())
}
