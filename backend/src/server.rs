// set up server
use actix_web::{web, App, HttpServer};

use crate::api;


pub struct Server {
    //inner: HttpServer<|| -> App<AppEntry>, App<AppEntry>, AppInit<AppEntry, BoxBody>, BoxBody>,
}

impl Server {
    // pub async fn new() -> Self {
    //     Self {
    //         inner: HttpServer::new(|| {
    //         App::new()
    //             .route("/", web::get().to(api::hello))
    //     })
    //     .bind(("127.0.0.1", 8080))?
    //     .run()
    //     .await
    //     }
    // }
}