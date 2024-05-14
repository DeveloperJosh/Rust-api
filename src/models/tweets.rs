use serde::Serialize;
use serde::Deserialize;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Clone)]
pub struct Tweet {
    pub id: i32,
    pub content: String,
    pub likes: Option<i32>,  // Change to Option<i32> if `likes` can be null
    pub created_at: chrono::NaiveDateTime,
}


#[derive(Serialize, Deserialize)]
pub struct TweetData {
    pub content: String,
}
