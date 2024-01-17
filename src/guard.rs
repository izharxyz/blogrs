use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};

use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;

use crate::{models::user::UserModel, schema::TokenClaims, AppState};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub async fn auth_guard_middleware(
    cookie_jar: CookieJar,
    State(data): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        })
        .ok_or_else(|| {
            let error_response = ErrorResponse {
                status: "fail",
                message: "You are not logged in, please login and try again".to_string(),
            };
            (StatusCode::UNAUTHORIZED, Json(error_response))
        })?;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(data.env.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| {
        let error_response = ErrorResponse {
            status: "fail",
            message: "Invalid token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(error_response))
    })?
    .claims;

    let user = sqlx::query_as!(
        UserModel,
        "SELECT * FROM users WHERE email = $1",
        claims.email
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|e| {
        tracing::error!("Error fetching user from database: {e:?}");
        let error_response = ErrorResponse {
            status: "fail",
            message: "Error fetching user from database".to_string(),
        };
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?
    .ok_or_else(|| {
        let error_response = ErrorResponse {
            status: "fail",
            message: "The user belonging to this token no longer exists".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(error_response))
    })?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
