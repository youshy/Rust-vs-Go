use bytes::Bytes;
use futures::future::ok;
use futures::stream::once;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)] // why do I have to do that?
struct Tweet {
    id: Uuid,
    author: String,
    body: String,
    likes: i64,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Sunshine!")
}

async fn get_tweets() -> impl Responder {
    // Mock response
    HttpResponse::NotImplemented().body("get_tweets not implemented")
}

async fn post_tweet(author: String, body: String) -> impl Responder {
    let _t = &Tweet {
        id: Uuid::new_v4(),
        author,
        body,
        likes: 0,
    }; // how to read body response?

    HttpResponse::NotImplemented().body("post_tweet not implemented")
}

async fn get_single_tweet() -> impl Responder {
    // Mock response
    HttpResponse::NotImplemented().body("get_single_tweet not implemented")
}

async fn delete_single_tweet() -> impl Responder {
    // Mock response
    HttpResponse::NotImplemented().body("delete_single_tweet not implemented")
}

async fn get_tweet_likes() -> impl Responder {
    // Mock response
    HttpResponse::NotImplemented().body("get_tweet_likes not implemented")
}

async fn post_tweet_like() -> impl Responder {
    // Mock response
    HttpResponse::NotImplemented().body("post_tweet_like not implemented")
}

async fn delete_tweet_like() -> impl Responder {
    // Mock response
    HttpResponse::NotImplemented().body("delete_tweet_like not implemented")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(index)).service(
            web::scope("/api")
                .route("/tweets/", web::get().to(get_tweets))
                .route("/tweets/", web::post().to(post_tweet))
                .route("/tweets/{id}/", web::get().to(get_single_tweet))
                .route("/tweets/{id}/", web::delete().to(delete_single_tweet))
                .route("/tweets/{id}/likes/", web::get().to(get_tweet_likes))
                .route("/tweets/{id}/likes/", web::post().to(post_tweet_like))
                .route("/tweets/{id}/likes/", web::delete().to(delete_tweet_like)),
        )
    })
    .bind("127.0.0.1:9000")?
    .run()
    .await
}
