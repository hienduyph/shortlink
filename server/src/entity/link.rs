use async_trait::async_trait;
use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;

use super::XError;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "links")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub shorten: String,
    pub link_type: i32,
    pub url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: i64,
    pub updated_by: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}



#[async_trait]
pub trait LinkQueryRepo: Send + Sync {
    async fn find_by_key(&self, input: &str) -> Result<Model, XError>;
}

#[async_trait]
pub trait LinkCtlRepo: Send + Sync {
    async fn create(&self, input: Model) -> Result<Model, XError>;
}
