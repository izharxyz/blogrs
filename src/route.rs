use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{controller::fetch_post_controller, AppState};

pub fn api_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/posts", get(fetch_post_controller))
        .with_state(app_state)
}
