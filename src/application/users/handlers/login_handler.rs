use crate::domain::users::repositories::UserRepositoryTrait;
use crate::application::users::commands::login_command::LoginCommand;
use crate::domain::users::services::verify_password;
use crate::infrastructure::jwt::token_jwt::generate_jwt;

pub struct LoginHandler<R: UserRepositoryTrait> {
	pub repo: R,
}

impl<R: UserRepositoryTrait> LoginHandler<R> {
	pub async fn handle(&self, cmd: LoginCommand) -> Result<String, String> {
		let user = self.repo.find_by_email(&cmd.email).await?;

		let is_valid = verify_password(&user.encrypted_password, &cmd.password)?;
		if !is_valid {
			return Err("Invalid credentials".into());
		}

		generate_jwt(user.id)
	}
}
