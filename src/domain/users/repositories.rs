use async_trait::async_trait;
use crate::domain::users::entities::Model as User;

#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
	async fn create(&self, user: User) -> Result<(), String>;
	async fn get_all(&self) -> Result<Vec<User>, String>;
	async fn get_user_by_id(&self, id: i64) -> Result<User, String>;
	async fn find_by_email(&self, email: &str) -> Result<User, String>; // <-- Agrega esto
}
