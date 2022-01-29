use actix_web::{web, App, HttpServer, Responder};
use std::time::Duration;
use tokio;

async fn my_handler() -> impl Responder {
    tokio::time::delay_for(Duration::from_secs(5)).await; // <-- Ok. Worker thread will handle other requests here
    "response"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(my_handler)))
        .workers(4)
        .bind("127.0.0.1:8080")?
        .run()
        .await // <- Start 4 workers
}
