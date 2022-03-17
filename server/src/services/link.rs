use std::sync::Arc;

use crate::entity::{link, user, XError};

pub struct LinkService {
    link_query: Arc<dyn link::LinkQueryRepo>,
}

impl LinkService {
    pub(crate) fn new(link_query: Arc<dyn link::LinkQueryRepo>) -> Self {
        Self { link_query }
    }
    pub(crate) async fn expand(&self, key: &str) -> Result<link::Model, XError> {
        self.link_query.find_by_key(key).await
    }
}
