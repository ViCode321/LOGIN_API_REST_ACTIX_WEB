use crate::domain::users::repositories::UserRepositoryTrait;
use crate::domain::users::entities::Model as User;

pub struct GetUserHandler<R: UserRepositoryTrait> {
	pub repo: R,
}

impl<R: UserRepositoryTrait> GetUserHandler<R> {
	pub async fn handle(&self) -> Result<Vec<User>, String> {
		self.repo.get_all().await
	}
}
