use std::sync::Arc;

use axum::{
    debug_handler,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    models::post::PostModel,
    schema::{CreatePostSchema, FetchAllPostSchema, FilterOptions, ParamOptions, UpdatePostSchema},
    AppState,
};

pub async fn fetch_post_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let post_query = sqlx::query_as!(
        FetchAllPostSchema,
        r#"
        SELECT id, title, slug, user_id, excerpt, category_id, created_at, updated_at FROM post
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

#[debug_handler]
pub async fn fetch_post_detail_handler(
    Path(params): Path<ParamOptions>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let post_slug = params.slug.unwrap();

    let post_query = sqlx::query_as!(
        PostModel,
        r#"
        SELECT * FROM post
        WHERE slug = $1
        "#,
        post_slug
    )
    .fetch_one(&data.db)
    .await;

    if post_query.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Post item with Slug: {} not found", post_slug),
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let post = post_query.unwrap();

    let response = serde_json::json!({
        "status": "success",
        "data": post,
    });

    Ok((StatusCode::OK, Json(response)))
}

#[debug_handler]
pub async fn create_post_handler(
    State(data): State<Arc<AppState>>,
    Json(payload): Json<CreatePostSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let title = payload.title;
    let slug = payload.slug;
    let excerpt = payload.excerpt;
    let content = payload.content;
    let category_id = payload.category_id.unwrap_or(1);
    let user_id = 1; // TODO: Change this to the logged in user

    let create_query = sqlx::query_as!(
        PostModel,
        r#"
        INSERT INTO post (title, slug, excerpt, content, category_id, user_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
        title,
        slug,
        excerpt,
        content,
        category_id,
        user_id
    )
    .fetch_one(&data.db)
    .await;

    match create_query {
        Ok(created_post) => {
            tracing::info!("Successfully created post with slug: {}", created_post.slug);
            let response = serde_json::json!({"status": "success","data": serde_json::json!({
                "post": created_post
            })});

            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "Post with that slug already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            tracing::error!("Error creating post: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    json!({"status": "fail","message": "Something bad happened while creating the post"}),
                ),
            ))
        }
    }
}

#[debug_handler]
pub async fn update_post_handler(
    Path(params): Path<ParamOptions>,
    State(data): State<Arc<AppState>>,
    Json(payload): Json<UpdatePostSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let post_slug = params.slug.unwrap();

    let post_query = sqlx::query_as!(
        PostModel,
        r#"
        SELECT * FROM post
        WHERE slug = $1
        "#,
        post_slug
    )
    .fetch_one(&data.db)
    .await;

    if post_query.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Post item with slug: {} not found", post_slug),
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
        WHERE slug = $6
        RETURNING *
        "#,
        title,
        slug,
        excerpt,
        content,
        category_id,
        post_slug
    )
    .fetch_one(&data.db)
    .await;

    match update_query {
        Ok(updated_post) => {
            tracing::info!("Successfully updated post with slug: {}", post_slug);
            let response = serde_json::json!({"status": "success","data": serde_json::json!({
                "post": updated_post
            })});

            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "Post with that slug already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            tracing::error!("Error updating post: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    json!({"status": "fail","message": "Something bad happened while updating the post"}),
                ),
            ))
        }
    }
}

#[debug_handler]
pub async fn delete_post_handler(
    Path(params): Path<ParamOptions>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let post_slug = params.slug.unwrap();

    let post_query = sqlx::query_as!(
        PostModel,
        r#"
        SELECT * FROM post
        WHERE slug = $1
        "#,
        post_slug
    )
    .fetch_one(&data.db)
    .await;

    if post_query.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Post item with slug: {} not found", post_slug),
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let delete_query = sqlx::query!(
        r#"
        DELETE FROM post
        WHERE slug = $1
        RETURNING *
        "#,
        post_slug
    )
    .fetch_one(&data.db)
    .await;

    match delete_query {
        Ok(_) => {
            tracing::info!("Successfully deleted post with slug: {}", post_slug);
            let response = serde_json::json!({"status": "success"});

            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => {
            tracing::error!("Error deleting post: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    json!({"status": "fail","message": "Something bad happened while deleting the post"}),
                ),
            ))
        }
    }
}
