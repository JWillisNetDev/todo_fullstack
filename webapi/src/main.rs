use crate::handlers::RouteTodoExt;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use std::net::SocketAddr;
use std::path::PathBuf;
use axum::response::Html;
use tokio::fs;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod db;
mod handlers;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug,hyper=info,mio=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = dotenv::var("DATABASE_URL")
        .expect("Could not find value for required environment variable DATABASE_URL.");

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    let pool = bb8::Pool::builder().build(config).await.unwrap();

    let static_dir = dotenv::var("STATIC_DIR").expect("Static directory not specified in environment variables.");
    let app = Router::new()
        .route_todo()
        .fallback_service(get(|req| async move {
            let res = ServeDir::new(&static_dir).oneshot(req).await.unwrap();
            let status = res.status();
            match status {
                StatusCode::NOT_FOUND => {
                    let index_path = PathBuf::from(&static_dir).join("index.html");
                    fs::read_to_string(index_path)
                        .await
                        .map(|index_content| (StatusCode::OK, Html(index_content)).into_response())
                        .unwrap_or_else(|_| {
                            (StatusCode::INTERNAL_SERVER_ERROR, "index.html not found").into_response()
                        })
                }
                _ => res.into_response(),
            }
        }))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(pool);

    let socket_addr = dotenv::var("HTTP_SERVE_ADDR")
        .unwrap_or("[::1]:8081".into())
        .parse::<SocketAddr>()
        .expect("Failed to convert server address into a valid socket address");

    tracing::info!("listening on {socket_addr:?}");

    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .expect("Unable to start server.");
}
