pub struct Email(String);

impl Email {
	pub fn new(value: String) -> Result<Self, String> {
		if value.contains('@') {
			Ok(Self(value))
		} else {
			Err("Email inválido".into())
		}
	}

	pub fn value(&self) -> &str {
		&self.0
	}
}
