mod accounts;
mod health;

use crate::entity::XError;

pub type Result<T> = std::result::Result<T, XError>;

pub(crate) use accounts::*;
pub(crate) use health::*;
