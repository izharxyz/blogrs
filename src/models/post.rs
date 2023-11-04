use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub content: String,
    // pub created_at: chrono::NaiveDateTime, TODO: Fix this
    // pub updated_at: chrono::NaiveDateTime, TODO: Fix this
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub author: String,
    pub content: String,
}
