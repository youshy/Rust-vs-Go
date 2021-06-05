use std::time::{SystemTime, UNIX_EPOCH};
use futures::future::ok;
use futures::stream::once;
use bytes::{Bytes};

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct File {
    name: String,
    time: u64,
    err: String
}

#[derive(Deserialize)]
struct Download {
    name: String,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Sunshine!")
}

async fn upload() -> impl Responder {
    println!("Uploading...");
    let u = &File {
        name: "dummy data".to_string(),
        time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        err: "".to_string()
    };

    HttpResponse::Ok().json(u)
}

async fn download(info: web::Path<Download>) -> HttpResponse {
    println!("Downloading...");
    let name = String::from(info.name.as_str());
    let body = once(ok::<_, actix_web::Error>(Bytes::from(name)));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(
                web::scope("/api")
                .route("/files/", web::post().to(upload))
                .route("/files/{name}/", web::get().to(download))
            )
    })
    .bind("127.0.0.1:9000")?
    .run()
    .await
}

