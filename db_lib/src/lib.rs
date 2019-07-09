#![feature(trivial_bounds)]
#[macro_use]
extern crate diesel;
extern crate diesel_derive_enum;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres@localhost:5432".to_string());
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}