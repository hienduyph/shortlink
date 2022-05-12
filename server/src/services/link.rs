use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use validator::Validate;

use crate::entity::{link, XError};

pub struct LinkService {
    link_query: Arc<dyn link::LinkQueryRepo>,
    link_create: Arc<dyn link::LinkCtlRepo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub(crate) struct LinkCreation {
    #[validate(url)]
    pub url: String,
    // the user that logged into this system
    pub user_id: Option<i64>,
    // the cookie_id or device_id that gen when user not login
    pub anonymous_id: Option<String>,
}

impl LinkService {
    pub(crate) fn new(
        link_query: Arc<dyn link::LinkQueryRepo>,
        link_create: Arc<dyn link::LinkCtlRepo>,
    ) -> Self {
        Self {
            link_query,
            link_create,
        }
    }

    pub(crate) async fn expand(&self, key: &str) -> Result<link::Model, XError> {
        self.link_query.find_by_key(key).await
    }

    pub(crate) async fn create(&self, form: &LinkCreation) -> Result<link::Model, XError> {
        let user_id = form.user_id.clone().unwrap_or_else(|| 0);
        form.validate()?;
        if form.anonymous_id.is_none() && user_id == 0 {
            return Err(XError::bad_request("ether anno or user must be required"));
        }

        // for mvp version, we concat the url and user that create user -> calc hash and get the
        let seed = form
            .user_id
            .map(|v| v.to_string())
            .or_else(|| form.anonymous_id.clone())
            .unwrap_or_else(|| "".to_string());

        let full_link = seed + "$$" + &form.url;
        let mut hasher = Sha256::new();
        hasher.update(full_link);
        let result = hasher.finalize();
        let c = base_62::encode(&result);
        // take first 7 chars
        let key = &c[..8];

        let mut model = link::Model {
            url: form.url.clone(),
            shorten: key.to_string(),
            ..link::Model::default()
        };
        if user_id > 0 {
            model.created_by = user_id;
            model.updated_by = user_id;
        }
        let v = self.link_create.create(model).await?;
        Ok(v)
    }

    pub fn hello() -> String {
        "My name is q".into()
    }
}
