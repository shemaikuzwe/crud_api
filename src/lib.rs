use std::env;

use diesel::{Connection, PgConnection};
use dotenv::dotenv;
pub mod models;
pub mod schema;
pub mod auth;
pub mod admin;
pub mod  shared;
pub mod middleware;

pub fn connect_db()->PgConnection{
 dotenv().ok();
 let database_url=env::var("DATABASE_URL").expect("DATABASE_URL not set");
 PgConnection::establish(&database_url).unwrap_or_else(|_|panic!("error connecting to database"))
}