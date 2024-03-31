use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};

pub mod models;
pub mod repositories;
pub mod schema;

pub type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;
