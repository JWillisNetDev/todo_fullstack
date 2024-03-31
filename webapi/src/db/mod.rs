use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode}
};
use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager, };

pub mod models;
pub mod schema;
pub mod repositories;

pub type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;