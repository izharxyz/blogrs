use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{self, IntoResponse},
    Json,
};
use serde_json::json;

use crate::{
    model::{CategoryModel, PostModel},
    schema::{CreatePostSchema, FilterOptions, ParamOptions, UpdatePostSchema},
    AppState,
};

pub async fn fetch_post_controller(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let post_query = sqlx::query_as!(
        PostModel,
        r#"
        SELECT * FROM post
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if post_query.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Something bad happened while fetching all note items",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let posts = post_query.unwrap();

    let response = serde_json::json!({
        "status": "success",
        "data": posts,
    });

    Ok((StatusCode::OK, Json(response)))
}
