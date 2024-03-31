use crate::db::{
    models::{Todo, UpdateTodo},
    repositories::DbConnection,
};
use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod db;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_diesel_async_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = dotenv::var("DATABASE_URL")
        .expect("Could not find value for required environment variable DATABASE_URL.");

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    let pool = bb8::Pool::builder().build(config).await.unwrap();

    let app = Router::new()
        .route("/todo/list", get(get_todos))
        .route("/todo/:id", get(get_todo))
        .route("/todo/create", post(create_todo))
        .route("/todo/:id", put(update_todo))
        .route("/todo/:id", delete(delete_todo))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn internal_error(err: String) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

async fn get_todos(db: DbConnection) -> Result<Json<Vec<Todo>>, (StatusCode, String)> {
    let todos = DbConnection::get_todos(db).await.map_err(internal_error)?;
    Ok(Json(todos))
}

async fn get_todo(
    db: DbConnection,
    Path(id): Path<i32>,
) -> Result<Json<Option<Todo>>, (StatusCode, String)> {
    let result = DbConnection::get_todo(db, id)
        .await
        .map_err(internal_error)?;
    Ok(Json(result))
}

async fn create_todo(db: DbConnection, title: String) -> Result<Json<Todo>, (StatusCode, String)> {
    let todo = DbConnection::create_todo(db, title.as_str())
        .await
        .map_err(internal_error)?;

    Ok(Json(todo))
}

async fn update_todo<'a>(
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

async fn delete_todo(db: DbConnection, Path(id): Path<i32>) -> Result<(), (StatusCode, String)> {
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
