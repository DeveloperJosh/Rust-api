// This code needs to be recoded to work with the new authentication system,
// I plan to do this later tonight

use log::error; // Import the error macro from the log crate

use actix_web::{web, FromRequest, HttpRequest, Error, dev::Payload, HttpResponse, Responder};
use futures::future::{ok, err, Ready};
use actix_web::error::ErrorBadRequest;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use crate::AppState; // Make sure to import AppState
use crate::models::tweets::{Tweet, TweetData};
use crate::models::users::Claims;

impl FromRequest for Claims {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(auth_header) = req.headers().get("Authorization") {
            let auth_str = auth_header.to_str().unwrap_or("");
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                let secret = std::env
                    ::var("JWT_SECRET")
                    .expect("JWT_SECRET must be set");
                match decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::new(Algorithm::HS256)) {
                    Ok(c) => return ok(c.claims),
                    Err(_) => return err(ErrorBadRequest("Invalid token").into()),
                }
            }
        }
        err(ErrorBadRequest("Missing Authorization Header").into())
    }
}

//post a tweet but only if the user is authenticated
pub async fn post_tweet(
    data: web::Data<AppState>,
    claims: Claims,
    tweet_data: web::Json<TweetData>,
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO tweets (content, user_id, created_at) VALUES ($1, $2, NOW()) RETURNING id",
        tweet_data.content,
        claims.id,
    )
    .fetch_one(&data.pool)
    .await;

    println!("User ID: {}", claims.id);

    match result {
        Ok(record) => HttpResponse::Ok().json(record.id),
        Err(e) => {
            error!("Database error: {:?}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        },
    }
}

//like a tweet but only if the user is authenticated
pub async fn like_tweet(
    data: web::Data<AppState>,
    _claims: Claims,
    like: web::Json<Tweet>,
) -> impl Responder {
    let result = sqlx::query!(
        "UPDATE tweets SET likes = likes + 1 WHERE id = $1",
        like.id
    )
    .execute(&data.pool)
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            error!("Database error: {:?}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        },
    }
}