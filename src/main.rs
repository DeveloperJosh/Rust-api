mod endpoints;
mod auth;
mod models;

use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use sqlx::postgres::{PgPoolOptions, PgPool};
//use serde::Deserialize;
use env_logger::Env;
use dotenv::dotenv;
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::auth::jwt_middleware;

use endpoints::{
    post_tweet, get_tweets, get_tweet, delete_tweet, update_tweet, like_tweet,
    register_user, login_user,
};

pub struct AppState {
    pub pool: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    //env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    println!("Connected to database");

    // Initialize database pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    let data = web::Data::new(AppState {
        pool: pool.clone(), // Clone the pool for the AppState
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE, actix_web::http::header::ACCEPT])
            .supports_credentials()
            .max_age(3600);

        let auth = HttpAuthentication::bearer(jwt_middleware);

        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("IP: {ip}, User-Agent: {user-agent}, Request: {request}"))
            .wrap(cors)
            .app_data(data.clone()) // Clone the data for each worker
            .service(
                web::scope("/api")
                    .wrap(auth)
                    .route("/create", web::post().to(post_tweet))
                    .route("/like", web::post().to(like_tweet))
                    .route("/all", web::get().to(get_tweets))
                    .route("/get/{id}", web::get().to(get_tweet))
                    .route("/delete/{id}", web::delete().to(delete_tweet))
                    .route("/update/{id}", web::put().to(update_tweet)),
            )
            .service(
                web::scope("/user")
                    .route("/register", web::post().to(register_user))
                    .route("/login", web::post().to(login_user)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
