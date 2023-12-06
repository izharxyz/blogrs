use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePostSchema {
    pub title: String,
    pub slug: String,
    pub author_id: Option<i32>,
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
