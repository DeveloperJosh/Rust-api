use serde::{Serialize, Deserialize};
use actix_web::{web, HttpResponse, Responder};
use mongodb::bson::{doc};
use uuid::Uuid;
use crate::AppState;

#[derive(Serialize, Deserialize, Clone)]
pub struct Like {
    pub tweet_id: String,  // This will contain the UUID as a string
}

// Increment likes count for a tweet
pub async fn like_tweet(like: web::Json<Like>, data: web::Data<AppState>) -> impl Responder {
    let tweet_id_str = &like.tweet_id;
    if let Ok(tweet_id) = Uuid::parse_str(tweet_id_str) {
        let collection = &data.tweet_collection;
        let update_result = collection.update_one(
            doc! { "id": tweet_id.to_string() }, // Convert Uuid to string for BSON
            doc! { "$inc": { "likes": 1 } },
            None
        ).await;

        match update_result {
            Ok(update) if update.matched_count == 1 => HttpResponse::Ok().finish(),
            Ok(_) => HttpResponse::NotFound().finish(),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
    } else {
        HttpResponse::BadRequest().body("Invalid tweet ID format")
    }
}
