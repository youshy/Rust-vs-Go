use actix_web::{HttpResponse, Responder};
use uuid::Uuid;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T> {
    pub results: Vec<T>,
}

// pub type Tweets = Response<Tweet>;

#[derive(Serialize, Deserialize)] // why do I have to do that?
struct Tweet {
    id: Uuid,
    author: String,
    body: String,
    likes: i64,
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Sunshine!")
}

pub async fn get_tweets() -> impl Responder {
    // Mock response
    HttpResponse::NotImplemented().body("get_tweets not implemented")
}

pub async fn post_tweet(author: String, body: String) -> impl Responder {
    let _t = &Tweet {
        id: Uuid::new_v4(),
        author,
        body,
        likes: 0,
    }; // how to read body response?

    HttpResponse::NotImplemented().body("post_tweet not implemented")
}

pub async fn get_single_tweet() -> impl Responder {
    // Mock response
    HttpResponse::NotImplemented().body("get_single_tweet not implemented")
}

pub async fn delete_single_tweet() -> impl Responder {
    // Mock response
    HttpResponse::NotImplemented().body("delete_single_tweet not implemented")
}

pub async fn get_tweet_likes() -> impl Responder {
    // Mock response
    HttpResponse::NotImplemented().body("get_tweet_likes not implemented")
}

pub async fn post_tweet_like() -> impl Responder {
    // Mock response
    HttpResponse::NotImplemented().body("post_tweet_like not implemented")
}

pub async fn delete_tweet_like() -> impl Responder {
    // Mock response
    HttpResponse::NotImplemented().body("delete_tweet_like not implemented")
}
