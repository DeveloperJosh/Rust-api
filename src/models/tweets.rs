use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Clone)]
pub struct Tweet {
    pub id: i32,
    pub user_id: i32,
    pub content: Option<String>,
    pub likes: i32,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct TweetData {
    pub content: String,
}