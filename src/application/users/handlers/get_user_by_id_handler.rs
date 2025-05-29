use crate::domain::users::repositories::UserRepositoryTrait;
use crate::domain::users::entities::Model as User;

pub struct GetUserByIdHandler<R: UserRepositoryTrait> {
	pub repo: R,
}

impl<R: UserRepositoryTrait> GetUserByIdHandler<R> {
	pub async fn handle(&self, id: i64) -> Result<User, String> {
		self.repo.get_user_by_id(id).await
	}
}
