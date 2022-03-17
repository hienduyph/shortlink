use async_trait::async_trait;
use chrono::{NaiveDateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use super::XError;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub status: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Default for Model {
    fn default() -> Self {
        Model {
            id: 0,
            first_name: "".to_string(),
            last_name: "".to_string(),
            email: "".to_string(),
            password: "".to_string(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            status: 0,
        }
    }
}

#[async_trait]
pub trait UserQueryRepo: Send + Sync {
    async fn get_by_email(&self, email: &str) -> Result<Model, XError>;

    async fn get_by_id(&self, id: i64) -> Result<Model, XError>;
}

#[async_trait]
pub trait UserModifierRepo: Send + Sync {
    async fn create(&self, user: Model) -> Result<Model, XError>;
}
