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
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1 OR username = $2)")
            .bind(payload.email.to_owned().to_ascii_lowercase())
            .bind(payload.username.to_owned())
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
                "message": "User already exists, please login",
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

#[debug_handler]
pub async fn login_user_handler(
    State(data): State<Arc<AppState>>,
    Json(payload): Json<LoginUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user = sqlx::query_as!(
        UserModel,
        "SELECT * FROM users WHERE email = $1",
        payload.email.to_ascii_lowercase()
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|e| {
        tracing::error!("Error fetching user from database: {e:?}");
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Error fetching user from database",
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?
    .ok_or_else(|| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Invalid email or password",
        });
        (StatusCode::BAD_REQUEST, Json(error_response))
    })?;

    let is_valid_passwd = match PasswordHash::new(&user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(payload.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid_passwd {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Invalid password",
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    let claims = TokenClaims {
        iat: chrono::Utc::now().timestamp() as usize,
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        email: user.email.to_owned(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    )
    .map_err(|e| {
        tracing::error!("Error while generating token: {e:?}");
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Error while generating token",
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let cookie = Cookie::build(("token", token.to_owned()))
        .path("/")
        .max_age(time::Duration::hours(24 * 7))
        .same_site(SameSite::Lax)
        .http_only(true);

    let mut response = Response::new(json!({"status": "success", "token": token}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok(response)
}

#[debug_handler]
pub async fn logout_user_handler(
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let cookie = Cookie::build(("token", ""))
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true);

    let mut response = Response::new(json!({"status": "success"}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}

#[debug_handler]
pub async fn current_user_handler(
    Extension(user): Extension<UserModel>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let response = serde_json::json!({
        "status":  "success",
        "data": serde_json::json!({
            "user": filter_user_data(&user)
        })
    });

    Ok(Json(response))
}
