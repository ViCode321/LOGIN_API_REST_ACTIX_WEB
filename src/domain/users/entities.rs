use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Default, Clone, Serialize, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)] pub id: i64,
    pub encrypted_password: String,
    #[sea_orm(unique)] pub email: String,
    pub segundo_apellido: Option<String>,
    pub primer_apellido: String,
    pub segundo_nombre: Option<String>,
    pub primer_nombre: String,
    pub updated_at: DateTime,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}