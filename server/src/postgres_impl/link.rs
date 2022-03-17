use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{entity::prelude::*, ActiveValue::NotSet};

use crate::entity::{link, XError};

pub struct LinkImpl {
    conn: Arc<super::DBconn>,
}

impl LinkImpl {
    pub fn new(db: Arc<super::DBconn>) -> Self {
        Self { conn: db }
    }
}

#[async_trait]
impl link::LinkCtlRepo for LinkImpl {
    async fn create(&self, input: link::Model) -> Result<link::Model, XError> {
        let mut model: link::ActiveModel = input.into();
        model.id = NotSet;

        let val: link::Model = model.insert(self.conn.as_ref()).await?;
        Ok(val)
    }
}

#[async_trait]
impl link::LinkQueryRepo for LinkImpl {
    async fn find_by_key(&self, key: &str) -> Result<link::Model, XError> {
        if let Some(model) = link::Entity::find()
            .filter(link::Column::Shorten.eq(key))
            .one(self.conn.as_ref())
            .await?
        {
            return Ok(model);
        }
        return Err(XError::notfound("link not found"));
    }
}
