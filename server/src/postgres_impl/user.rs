use std::sync::Arc;

use async_trait::async_trait;

use crate::entity::{user, XError};

pub struct UserQueryImpl {
    db: Arc<super::DBconn>,
}

impl UserQueryImpl {
    pub fn new(db: Arc<super::DBconn>) -> Self {
        UserQueryImpl { db }
    }
}

#[async_trait]
impl user::UserQueryRepo for UserQueryImpl {
    async fn get_by_email(&self, email: &str) -> Result<user::Model, XError> {
        // let conn = self.db.get().unwrap();
        panic!("HH");
    }

    async fn get_by_id(&self, id: i64) -> Result<user::Model, XError> {
        panic!("impl")
    }
}
