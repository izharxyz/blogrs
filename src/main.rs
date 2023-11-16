use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod controllers;
mod models;

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "blogrs=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("can't connect to database");

    let app = Router::new()
        .route("/", get(|| async { "Welcome to blogrs API!" }))
        .route("/post/create", post(|| async { "Create Post" }))
        .route("/post/edit/:id", patch(|| async { "Edit Post" }))
        .route("/post/delete/:id", delete(|| async { "Delete Post" }))
        .with_state(pool);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
