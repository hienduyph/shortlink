#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod apps;
pub mod entity;
pub mod handlers;

mod diesel_impl;
mod infra;
mod schema;
