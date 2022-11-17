use actix_web::{App, HttpServer};
use actix_cors::Cors;

mod models;
mod schema;
mod dbconnector;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(api::list_rants)
            .service(api::create_rant)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}