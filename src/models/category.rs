use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct CategoryModel {
    pub id: i32,
    pub name: String,
}
