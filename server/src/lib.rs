#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod apps;
pub mod handlers;
pub mod entity;

mod schema;
mod diesel_impl;
mod infra;
