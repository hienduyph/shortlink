use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{entity::prelude::*};

use crate::entity::{user, user::Entity as User, XError};

pub struct UserQueryPostgresImpl {
    db: Arc<super::DBconn>,
}

impl UserQueryPostgresImpl {
    pub fn new(db: Arc<super::DBconn>) -> Self {
        UserQueryPostgresImpl { db }
    }
}

#[async_trait]
impl user::UserQueryRepo for UserQueryPostgresImpl {
    async fn get_by_email(&self, email: &str) -> Result<user::Model, XError> {
        let conn = self.db.as_ref();
        if let Some(u) = User::find()
            .filter(user::Column::Email.eq(email))
            .one(conn)
            .await?
        {
            return Ok(u);
        }
        return Err(XError::notfound("user not found"));
    }

    async fn get_by_id(&self, id: i64) -> Result<user::Model, XError> {
        if let Some(u) = User::find_by_id(id)
            .one(self.db.as_ref())
            .await? {
                return Ok(u);
            }
        return Err(XError::notfound("user not found"));
    }
}

pub struct UserModifierPostgresImpl {
    db: Arc<super::DBconn>,
}

impl UserModifierPostgresImpl {
    pub fn new(db: Arc<super::DBconn>) -> Self {
        UserModifierPostgresImpl{ db }
    }
}

#[async_trait]
impl user::UserModifierRepo for UserModifierPostgresImpl{
    async fn create(&self, user: user::Model) -> Result<user::Model, XError> {
        let amodel: user::ActiveModel = user.into();
        let u: user::Model = amodel.insert(self.db.as_ref()).await?;
        Ok(u)
    }
}
