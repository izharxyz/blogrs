use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct PostModel {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub author: Option<String>,
    pub excerpt: String,
    pub content: String,
    pub category: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct CategoryModel {
    pub id: i32,
    pub name: String,
}
