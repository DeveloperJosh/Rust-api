use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use crate::AppState; // Make sure to import AppState

#[derive(Serialize, Deserialize, Clone)]
pub struct Tweet {
    pub id: i32,
    pub content: String,
    pub likes: Option<i32>,  // Change to Option<i32> if `likes` can be null
    pub created_at: chrono::NaiveDateTime,
    pub author_id: i32,
    pub author_username: String,
}


#[derive(Serialize, Deserialize)]
pub struct TweetData {
    pub content: String,
}

// Post a tweet
pub async fn post_tweet(
    data: web::Data<AppState>, // Changed to use AppState
    tweet_data: web::Json<TweetData>
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO tweets (content, likes, created_at) VALUES ($1, 0, NOW()) RETURNING id",
        tweet_data.content,
    )
    .fetch_one(&data.pool) // Access the pool through AppState
    .await;

    match result {
        Ok(record) => HttpResponse::Ok().json(record.id),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// Get all tweets
pub async fn get_tweets(data: web::Data<AppState>) -> impl Responder {
    let tweets = sqlx::query_as!(
        Tweet,
        "SELECT * FROM tweets"
    )
    .fetch_all(&data.pool)
    .await;

    match tweets {
        Ok(tweets) => HttpResponse::Ok().json(tweets),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// Get a specific tweet
pub async fn get_tweet(data: web::Data<AppState>, tweet_id: web::Path<i32>) -> impl Responder {
    let tweet = sqlx::query_as!(
        Tweet,
        "SELECT * FROM tweets WHERE id = $1",
        tweet_id.into_inner()
    )
    .fetch_one(&data.pool)
    .await;

    match tweet {
        Ok(tweet) => HttpResponse::Ok().json(tweet),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn delete_tweet(data: web::Data<AppState>, tweet_id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query!(
        "DELETE FROM tweets WHERE id = $1",
        tweet_id.into_inner()
    )
    .execute(&data.pool)
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn update_tweet(
    data: web::Data<AppState>,
    tweet_id: web::Path<i32>,
    tweet_data: web::Json<TweetData>
) -> impl Responder {

    // add (edited) to the content
    let tweet_data = TweetData {
        content: format!("{} (edited)", tweet_data.content)
    };

    let result = sqlx::query!(
        "UPDATE tweets SET content = $1 WHERE id = $2",
        tweet_data.content,
        tweet_id.into_inner()
    )
    .execute(&data.pool)
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}