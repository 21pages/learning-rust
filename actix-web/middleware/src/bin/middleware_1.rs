use actix_service::Service;
use actix_web::{web, App, HttpServer};
use futures::future::FutureExt;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap_fn(|req, srv| {
                println!("Hi from start. You requested: {}", req.path());
                srv.call(req).map(|res| {
                    println!("Hi from response");
                    res
                })
            })
            .route(
                "/index.html",
                web::get().to(|| async { "Hello, middleware!" }),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
