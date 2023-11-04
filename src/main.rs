use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5432")
        .await
        .expect("can't connect to database");

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/users", post(|| async { "Create user" }))
        .route("/users/:id", patch(|| async { "Update user" }))
        .route("/users/:id", delete(|| async { "Delete user" }))
        .with_state(pool);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
