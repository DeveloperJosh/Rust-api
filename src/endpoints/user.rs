use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use crate::AppState;
use std::env;
use log::error;
use crate::models::users::{User, RegisterUserData, LoginUserData, Claims};

pub async fn register_user(
    data: web::Data<AppState>,
    user_data: web::Json<RegisterUserData>
) -> impl Responder {
    match hash(&user_data.password, DEFAULT_COST) {
        Ok(password_hash) => {
            let result = sqlx::query!(
                "INSERT INTO users (username, email, password_hash, created_at) VALUES ($1, $2, $3, NOW()) RETURNING id",
                user_data.username,
                user_data.email,
                password_hash,
            )
            .fetch_one(&data.pool)
            .await;

            match result {
                Ok(record) => HttpResponse::Ok().json(record.id),
                Err(e) => {
                    // if the email/username is already in use
                    // check user name and email
                    if e.to_string().contains("users_email_key") {
                        HttpResponse::Conflict().body("Email already in use")
                    } else if e.to_string().contains("users_username_key") {
                        HttpResponse::Conflict().body("Username already in use")
                    } else {
                        error!("Database error: {:?}", e);
                    HttpResponse::InternalServerError().body("Internal server error")
                    }
                },
            }
        },
        Err(e) => {
            error!("Hashing error: {:?}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        },
    }
}

pub async fn login_user(
    data: web::Data<AppState>,
    user_data: web::Json<LoginUserData>
) -> impl Responder {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1",
        user_data.email
    )
    .fetch_one(&data.pool)
    .await;

    match user {
        Ok(user) => {
            if verify(&user_data.password, &user.password_hash).unwrap_or(false) {
                let claims = Claims {
                    sub: user.email,
                    id: user.id, // Add this line
                    exp: (Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
                };
                let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret".to_string());
                match encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())) {
                    Ok(token) => HttpResponse::Ok().json(token),
                    Err(e) => {
                        error!("Token encoding error: {:?}", e);
                        HttpResponse::InternalServerError().body("Internal server error")
                    }
                }
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        },
        Err(_) => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
}