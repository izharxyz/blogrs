use std::sync::Arc;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::{
    handlers::{
        auth::register_user_handler,
        post::{
            create_post_handler, delete_post_handler, fetch_post_detail_handler,
            fetch_post_handler, update_post_handler,
        },
    },
    AppState,
};

pub fn api_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/post", get(fetch_post_handler))
        .route("/post/:slug", get(fetch_post_detail_handler))
        .route("/post/create", post(create_post_handler))
        .route("/post/update/:slug", patch(update_post_handler))
        .route("/post/delete/:slug", delete(delete_post_handler))
        .route("/auth/register", post(register_user_handler))
        .with_state(app_state)
}
