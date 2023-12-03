use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct PostModel {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub author_id: Option<i32>,
    pub excerpt: String,
    pub content: String,
    pub category_id: Option<i32>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct CategoryModel {
    pub id: i32,
    pub name: String,
}
