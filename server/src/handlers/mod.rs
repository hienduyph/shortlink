mod accounts;
mod health;
mod link;

use crate::entity::XError;

pub type Result<T> = std::result::Result<T, XError>;

pub use accounts::*;
pub use health::*;
pub use link::*;
