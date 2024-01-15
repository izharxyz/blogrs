use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct UserModel {
    pub id: i32,
    pub name: Option<String>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
