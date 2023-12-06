use std::sync::Arc;

use axum::{
    routing::{get, patch, post},
    Router,
};

use crate::{
    controller::{create_post_controller, fetch_post_controller, update_post_controller},
    AppState,
};

pub fn api_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/post", get(fetch_post_controller))
        .route("/post/create", post(create_post_controller))
        .route("/post/update/:id", patch(update_post_controller))
        .with_state(app_state)
}
