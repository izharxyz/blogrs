mod guard;
mod handlers;
mod model;
mod route;
mod schema;

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
    routing::get,
    Router,
};
use std::sync::Arc;

use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::net::TcpListener;

use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use route::api_routes;

pub struct Env {
    jwt_secret: String,
}

pub struct AppState {
    db: Pool<Postgres>,
    env: Env,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET env variable must be set");

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "blogrs=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = match PgPoolOptions::new()
        .max_connections(50)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            tracing::info!("Successfully connected to the database!");
            pool
        }
        Err(err) => {
            tracing::error!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origin(Any)
        .allow_credentials(false)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app_state = Arc::new(AppState {
        db: pool.clone(),
        env: Env { jwt_secret },
    });
    let app = Router::new()
        .route("/", get(|| async { "Welcome to blogrs API!" }))
        .nest("/api", api_routes(app_state))
        .layer(cors);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
