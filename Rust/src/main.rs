mod handlers;

use actix_web::{web, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(handlers::index))
            .service(
                web::scope("/api")
                    .route("/tweets/", web::get().to(handlers::get_tweets))
                    .route("/tweets/", web::post().to(handlers::post_tweet))
                    .route("/tweets/{id}/", web::get().to(handlers::get_single_tweet))
                    .route(
                        "/tweets/{id}/",
                        web::delete().to(handlers::delete_single_tweet),
                    )
                    .route(
                        "/tweets/{id}/likes/",
                        web::get().to(handlers::get_tweet_likes),
                    )
                    .route(
                        "/tweets/{id}/likes/",
                        web::post().to(handlers::post_tweet_like),
                    )
                    .route(
                        "/tweets/{id}/likes/",
                        web::delete().to(handlers::delete_tweet_like),
                    ),
            )
    })
    .bind("127.0.0.1:9000")?
    .run()
    .await
}
