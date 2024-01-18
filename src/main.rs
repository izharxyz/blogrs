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

use sqlx::PgPool;

use tower_http::cors::{Any, CorsLayer};

use route::api_routes;

pub struct Env {
    jwt_secret: String,
}

pub struct AppState {
    db: PgPool,
    env: Env,
}

#[shuttle_runtime::main]
pub async fn axum(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_secrets::Secrets] secrets: shuttle_secrets::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Migrations failed :(");

    let jwt_secret = secrets.get("JWT_SECRET").expect("JWT_SECRET must be set");

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

    Ok(app.into())
}
