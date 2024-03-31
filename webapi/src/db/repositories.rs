use crate::db::{
    models::{CreateTodo, Todo, UpdateTodo},
    Pool,
};
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use diesel::prelude::*;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};

pub struct DbConnection(
    bb8::PooledConnection<'static, AsyncDieselConnectionManager<AsyncPgConnection>>,
);

pub fn map_internal_err<E: std::error::Error>(err: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

fn map_err<E: std::error::Error>(err: E) -> String {
    err.to_string()
}

#[async_trait]
impl<S> FromRequestParts<S> for DbConnection
where
    S: Send + Sync,
    Pool: FromRef<S>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = Pool::from_ref(state);
        let conn = pool.get_owned().await.map_err(map_internal_err)?;

        Ok(Self(conn))
    }
}

impl DbConnection {
    pub async fn get_todos(DbConnection(mut conn): Self) -> Result<Vec<Todo>, String> {
        super::schema::todo::table
            .select(Todo::as_select())
            .load(&mut conn)
            .await
            .map_err(map_err)
    }

    pub async fn get_todo(DbConnection(mut conn): Self, id: i32) -> Result<Option<Todo>, String> {
        super::schema::todo::table
            .find(id)
            .select(Todo::as_select())
            .first(&mut conn)
            .await
            .optional()
            .map_err(map_err)
    }

    pub async fn create_todo(DbConnection(mut conn): Self, title: &str) -> Result<Todo, String> {
        let new_todo = CreateTodo { title };
        diesel::insert_into(super::schema::todo::table)
            .values(&new_todo)
            .returning(Todo::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(map_err)
    }
    pub async fn update_todo<'a>(
        DbConnection(mut conn): Self,
        id: i32,
        update_todo: UpdateTodo,
    ) -> Result<Option<Todo>, String> {
        diesel::update(super::schema::todo::table)
            .filter(super::schema::todo::id.eq(id))
            .set(&update_todo)
            .returning(Todo::as_returning())
            .get_result(&mut conn)
            .await
            .optional()
            .map_err(map_err)
    }

    pub async fn delete_todo(DbConnection(mut conn): Self, id: i32) -> Result<bool, String> {
        let deleted_count = diesel::delete(super::schema::todo::table)
            .filter(super::schema::todo::id.eq(id))
            .execute(&mut conn)
            .await
            .map_err(map_err)?;

        Ok(deleted_count > 0)
    }
}
