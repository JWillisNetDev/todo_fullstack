use crate::db::{
    models::{Todo, UpdateTodo},
    repositories::DbConnection,
    Pool,
};
use axum::{
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};

fn internal_error(err: String) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

pub trait RouteTodoExt {
    fn route_todo(&self) -> Self;
}

impl RouteTodoExt for Router<Pool> {
    fn route_todo(&self) -> Self {
        self.to_owned()
            .route("/api/todo/list", get(get_todos))
            .route("/api/todo/:id", get(get_todo))
            .route("/api/todo/create", post(create_todo))
            .route("/api/todo/:id", put(update_todo))
            .route("/api/todo/:id", delete(delete_todo))
    }
}

pub async fn get_todos(db: DbConnection) -> Result<Json<Vec<Todo>>, (StatusCode, String)> {
    let todos = DbConnection::get_todos(db).await.map_err(internal_error)?;
    Ok(Json(todos))
}

pub async fn get_todo(
    db: DbConnection,
    Path(id): Path<i32>,
) -> Result<Json<Option<Todo>>, (StatusCode, String)> {
    let result = DbConnection::get_todo(db, id)
        .await
        .map_err(internal_error)?;
    Ok(Json(result))
}

pub async fn create_todo(
    db: DbConnection,
    title: String,
) -> Result<Json<Todo>, (StatusCode, String)> {
    let todo = DbConnection::create_todo(db, title.as_str())
        .await
        .map_err(internal_error)?;

    Ok(Json(todo))
}

pub async fn update_todo(
    db: DbConnection,
    Path(id): Path<i32>,
    Json(update_todo): Json<UpdateTodo>,
) -> Result<Json<Option<Todo>>, (StatusCode, String)> {
    let todo = DbConnection::update_todo(db, id, update_todo)
        .await
        .map_err(internal_error)?;

    if todo.is_none() {
        Err((
            StatusCode::NOT_FOUND,
            format!("No todo item exists with id `{id}`"),
        ))
    } else {
        Ok(Json(todo))
    }
}

pub async fn delete_todo(
    db: DbConnection,
    Path(id): Path<i32>,
) -> Result<(), (StatusCode, String)> {
    let res = DbConnection::delete_todo(db, id)
        .await
        .map_err(internal_error)?;
    if res {
        Ok(())
    } else {
        Err((
            StatusCode::NOT_FOUND,
            format!("No todo item exists with id `{id}` or todo item could not be deleted."),
        ))
    }
}
