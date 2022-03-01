#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod apps;
pub mod entity;
pub mod handlers;
pub mod services;
pub mod utils;

mod diesel_impl;
mod infra;
mod schema;
