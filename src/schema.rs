use serde::{Deserialize, Serialize};

// Post related schemas
#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: Option<i32>,
    pub slug: Option<String>,
}

// this is the schema for the the post overview; it is used to fetch all posts so it doesn't need the content
#[derive(Debug, Deserialize, Serialize)]
pub struct FetchAllPostSchema {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub user_id: Option<i32>,
    pub excerpt: String,
    pub category_id: Option<i32>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePostSchema {
    pub title: String,
    pub slug: String,
    pub user_id: Option<i32>,
    pub excerpt: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePostSchema {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub excerpt: Option<String>,
    pub content: Option<String>,
    pub category_id: Option<i32>,
}

// Auth related schemas
// user data schema is for response data so it doesn't include password.
#[derive(Serialize, Debug)]
pub struct UserDataSchema {
    pub id: i32,
    pub name: Option<String>,
    pub username: String,
    pub email: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Deserialize, Debug)]
pub struct RegisterUserSchema {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}
