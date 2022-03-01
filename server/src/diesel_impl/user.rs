use async_trait::async_trait;

use crate::entity::{User, UserQueryRepo, XError};

pub struct UserQueryImpl {}

impl UserQueryImpl {
    pub fn new() -> Self {
        UserQueryImpl {}
    }
}

#[async_trait]
impl UserQueryRepo for UserQueryImpl {
    async fn get_by_email(&self, email: &str) -> Result<User, XError> {
        panic!("impl")
    }

    async fn get_by_id(&self, id: i64) -> Result<User, XError> {
        panic!("impl")
    }
}
