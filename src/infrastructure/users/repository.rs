use sea_orm::{DatabaseConnection, ActiveModelTrait, Set};
use crate::domain::users::entities::Model as User;
use crate::domain::users::repositories::UserRepositoryTrait;
use crate::domain::users::entities;
use async_trait::async_trait;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};

pub struct UserRepositorySeaOrm {
	pub db: DatabaseConnection,
}

#[async_trait]
impl UserRepositoryTrait for UserRepositorySeaOrm {
	async fn create(&self, user: User) -> Result<(), String> {
		let model = entities::ActiveModel {
			encrypted_password: Set(user.encrypted_password),
			email: Set(user.email),
			segundo_apellido: Set(user.segundo_apellido),
			primer_apellido: Set(user.primer_apellido),
			segundo_nombre: Set(user.segundo_nombre),
			primer_nombre: Set(user.primer_nombre),
			updated_at: Set(user.updated_at),
			created_at: Set(user.created_at),
			..Default::default()
		};

		model.insert(&self.db).await.map_err(|e| e.to_string())?;
		Ok(())
	}

	async fn get_all(&self) -> Result<Vec<User>, String> {
		use sea_orm::EntityTrait;
		let users = entities::Entity::find()
			.all(&self.db)
			.await
			.map_err(|e| e.to_string())?;

		let result = users
			.into_iter()
			.map(|u| User {
				id: u.id,
				email: u.email,
				..Default::default()
			})
			.collect();

		Ok(result)
	}

	async fn get_user_by_id(&self, id: i64) -> Result<User, String> {
		let user = entities::Entity::find()
			.filter(entities::Column::Id.eq(id))
			.one(&self.db)
			.await
			.map_err(|e| e.to_string())?;

		match user {
			Some(u) => Ok(User {
				id: u.id,
				email: u.email,
				..Default::default()
			}),
			None => Err("User not found".to_string()),
		}
	}

	async fn find_by_email(&self, email: &str) -> Result<User, String> {

		let user = entities::Entity::find()
			.filter(entities::Column::Email.eq(email))
			.one(&self.db)
			.await
			.map_err(|e| e.to_string())?;

		user.ok_or_else(|| "User not found".to_string())
	}
}
