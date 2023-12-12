use std::sync::Arc;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::{
    handler::{create_post_handler, delete_post_handler, fetch_post_handler, update_post_handler},
    AppState,
};

pub fn api_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/post", get(fetch_post_handler))
        .route("/post/create", post(create_post_handler))
        .route("/post/update/:id", patch(update_post_handler))
        .route("/post/delete/:id", delete(delete_post_handler))
        .with_state(app_state)
}
