use crate::domain::users::entities::Model as User;
use crate::application::users::commands::register_command::RegisterCommand;
use crate::domain::users::repositories::UserRepositoryTrait;
use chrono::Utc;
pub struct RegisterHandler<R: UserRepositoryTrait> {
	pub repo: R,
}

impl<R: UserRepositoryTrait> RegisterHandler<R> {
	pub async fn handle(&self, cmd: RegisterCommand) -> Result<User, String> {
		let hashed = bcrypt::hash(&cmd.encrypted_password, 4).map_err(|e| e.to_string())?;

		let user = User {
			id: 0,
			encrypted_password: hashed,
			email: cmd.email,
			segundo_apellido: cmd.segundo_apellido,
			primer_apellido: cmd.primer_apellido,
			segundo_nombre: cmd.segundo_nombre,
			primer_nombre: cmd.primer_nombre,
			updated_at: Utc::now().naive_utc(),
			created_at: Utc::now().naive_utc(),
		};

		self.repo.create(user.clone()).await.map_err(|e| e.to_string())?;
		Ok(user)
	}
}
