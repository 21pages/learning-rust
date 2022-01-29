use actix_web::{get, web};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

/*
http://127.0.0.1:8080/?username=zhangsan
*/

// this handler get called only if the request's query contains `username` field
#[get("/")]
async fn index(info: web::Query<Info>) -> String {
    format!("Welcome {}!", info.username)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
