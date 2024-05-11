use actix_web::{web, HttpResponse, Responder};
use mongodb::{bson::doc, Collection};
use serde::{Serialize, Deserialize};
use crate::AppState;
use uuid::Uuid;  // Import the UUID crate

#[derive(Serialize, Deserialize, Clone)]
pub struct Tweet {
    pub id: String,  // Changed to String to store UUIDs as strings
    pub content: String,
    pub likes: u32,
}

#[derive(Serialize, Deserialize)]
pub struct TweetData {
    pub content: String,
}

// Post a tweet with a UUID
pub async fn post_tweet(data: web::Data<AppState>, tweet_data: web::Json<TweetData>) -> impl Responder {
    let collection = &data.tweet_collection;

    // Generate a new UUID for the tweet
    let new_id = Uuid::new_v4().to_string();

    let new_tweet = Tweet {
        id: new_id,
        content: tweet_data.into_inner().content,
        likes: 0,
    };

    let insert_result = collection.insert_one(new_tweet.clone(), None).await;

    match insert_result {
        Ok(_) => HttpResponse::Created().json(new_tweet),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
