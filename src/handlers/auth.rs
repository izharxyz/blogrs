use std::sync::Arc;

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{
    debug_handler,
    extract::State,
    http::{header, Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand_core::OsRng;
use serde_json::json;

use crate::{
    models::user::UserModel,
    schema::{LoginUserSchema, RegisterUserSchema, TokenClaims, UserDataSchema},
    AppState,
};

fn filter_user_data(user: &UserModel) -> UserDataSchema {
    UserDataSchema {
        id: user.id,
        name: user.name.to_owned(),
        username: user.username.to_string(),
        email: user.email.to_string(),
        created_at: user.created_at.to_owned(),
        updated_at: user.updated_at.to_owned(),
    }
}

#[debug_handler]
pub async fn register_user_handler(
    State(data): State<Arc<AppState>>,
    Json(payload): Json<RegisterUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_exists: Option<bool> =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
            .bind(payload.email.to_owned().to_ascii_lowercase())
            .fetch_one(&data.db)
            .await
            .map_err(|e| {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": format!("Database error: {}", e),
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            })?;

    if let Some(exists) = user_exists {
        if exists {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "User with that email already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_passwd = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| {
            tracing::error!("Error while hashing password: {e:?}");
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "Error while hashing password",
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })
        .map(|hash| hash.to_string())?;

    let user = sqlx::query_as!(
        UserModel,
        "INSERT INTO users (username,email,password) VALUES ($1, $2, $3) RETURNING *",
        payload.username,
        payload.email.to_ascii_lowercase(),
        hashed_passwd
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {e:?}");
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Database error",
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let response = serde_json::json!({"status": "success","data": serde_json::json!({
        "user": filter_user_data(&user)
    })});
    Ok(Json(response))
}
