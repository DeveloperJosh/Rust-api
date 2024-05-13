use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::AppState;  // Import AppState from your main.rs or the module where it's defined

#[derive(Serialize, Deserialize)]
pub struct Like {
    pub id: i32,  // Assuming an integer ID for SQL
}

// Increment likes count for a tweet
pub async fn like_tweet(data: web::Data<AppState>, like: web::Json<Like>) -> impl Responder {
    let result = sqlx::query!(
        "UPDATE tweets SET likes = likes + 1 WHERE id = $1",
        like.id
    )
    .execute(&data.pool)  // Use the pool from AppState
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
