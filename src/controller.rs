use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    model::PostModel,
    schema::{FilterOptions, ParamOptions, UpdatePostSchema},
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

pub async fn update_post_controller(
    Path(params): Path<ParamOptions>,
    Json(payload): Json<UpdatePostSchema>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let post_id = params.id.parse::<i32>().unwrap();

    let post_query = sqlx::query_as!(
        PostModel,
        r#"
        SELECT * FROM post
        WHERE id = $1
        "#,
        post_id
    )
    .fetch_one(&data.db)
    .await;

    if post_query.is_err() {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Post item with ID: {} not found", post_id),
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let post = post_query.unwrap();

    let title = payload.title.unwrap_or(post.title);
    let slug = payload.slug.unwrap_or(post.slug);
    let excerpt = payload.excerpt.unwrap_or(post.excerpt);
    let content = payload.content.unwrap_or(post.content);
    let category_id = payload.category_id.unwrap_or(post.category_id.unwrap_or(1));

    let update_query = sqlx::query_as!(
        PostModel,
        r#"
        UPDATE post
        SET title = $1, slug = $2, excerpt = $3, content = $4, category_id = $5
        WHERE id = $6
        RETURNING *
        "#,
        title,
        slug,
        excerpt,
        content,
        category_id,
        post_id
    )
    .fetch_one(&data.db)
    .await;

    match update_query {
        Ok(updated_post) => {
            let response = serde_json::json!({"status": "success","data": serde_json::json!({
                "post": updated_post
            })});

            return Ok((StatusCode::OK, Json(response)));
        }
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    json!({"status": "error","message": "Something bad happened while updating the post"}),
                ),
            ));
        }
    }
}
