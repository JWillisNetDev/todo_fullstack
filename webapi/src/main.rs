use crate::handlers::RouteTodoExt;
use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::{get},
    Router,
};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::fs;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod db;
mod handlers;

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
        .route_todo()
        .fallback_service(get(|request| async move {
            let static_dir = dotenv::var("STATIC_DIR").unwrap();
            match ServeDir::new(&static_dir).oneshot(request).await {
                Ok(resp) => {
                    if resp.status() == StatusCode::NOT_FOUND {
                        let index_path = PathBuf::from(&static_dir).join("index.html");
                        let index_content = match fs::read_to_string(index_path).await {
                            Err(_) => {
                                return Response::builder()
                                    .status(StatusCode::NOT_FOUND)
                                    .body(Body::from("Index file was not found."))
                                    .unwrap();
                            }
                            Ok(index_content) => index_content,
                        };

                        Response::builder()
                            .status(StatusCode::OK)
                            .body(Body::from(index_content))
                            .unwrap()
                    } else {
                        resp.into_response()
                    }
                }

                Err(err) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(format!("error: {err}")))
                    .expect("Error response.")
                    .into_response(),
            }
        }))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(pool);

    let socket_addr = dotenv::var("HTTP_SERVE_ADDR")
        .unwrap_or("[::1]:8080".into())
        .parse::<SocketAddr>()
        .expect("Failed to convert server address into a valid socket address");

    tracing::info!("listening on {socket_addr:?}");

    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .expect("Unable to start server.");
}
