use actix_web::{web, HttpResponse, Responder};
use mongodb::{bson::doc, Collection};
use serde::{Serialize, Deserialize};
use crate::AppState;
use uuid::Uuid;  // Import the UUID crate
use futures::stream::StreamExt; // Import StreamExt for handling streams


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

// Get all tweets 
pub async fn get_tweets(data: web::Data<AppState>) -> impl Responder {
    let collection = &data.tweet_collection;
    let find_result = collection.find(None, None).await;

    match find_result {
        Ok(cursor) => {
            let tweets: Vec<Tweet> = cursor
                .filter_map(|item| async {
                    match item {
                        Ok(tweet) => Some(tweet),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            None
                        }
                    }
                })
                .collect::<Vec<Tweet>>() // Collect tweets into a vector
                .await; // Await the collection process

            HttpResponse::Ok().json(tweets) // Send the vector as a JSON response
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}