mod endpoints;

use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use mongodb::{Client, options::ClientOptions, Collection};
use serde::Deserialize;
use endpoints::{Tweet, Like, post_tweet, get_tweets, like_tweet, delete_tweet, update_tweet};

use dotenv::dotenv;

pub struct AppState {
    pub tweet_collection: Collection<Tweet>,
    pub like_collection: Collection<Like>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    //let client_options = ClientOptions::parse("").await.unwrap();
    let client_options = ClientOptions::parse(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("test");

    let tweet_collection = db.collection::<Tweet>("tweets");
    let like_collection = db.collection::<Like>("likes");

    let app_data = web::Data::new(AppState {
        tweet_collection,
        like_collection,
    });

    HttpServer::new(move || {

        let cors = Cors::default()
        .allowed_origin("http://localhost:3000")  // Allow only from this origin
        .allowed_methods(vec!["GET", "POST"])    // Specify allowed methods
        .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE, actix_web::http::header::ACCEPT])
        .supports_credentials()
        .max_age(3600);


        App::new()
            .wrap(cors)
            .app_data(app_data.clone())
            .route("/api/create", web::post().to(post_tweet))
            // /api/delete/:id
            .route("/api/delete/{id}", web::delete().to(delete_tweet))
            // /api/update/:id
            .route("/api/update/{id}", web::put().to(update_tweet))
            .route("/api/all", web::get().to(get_tweets))
            .route("/api/like", web::post().to(like_tweet))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
