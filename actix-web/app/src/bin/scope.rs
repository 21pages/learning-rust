use actix_web::{web, App, HttpRequest, HttpServer};

async fn show_users(_req: HttpRequest) -> &'static str {
    "Welcome!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::scope("/users").service(web::resource("/test1").to(show_users)))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
}

/*
http://127.0.0.1:8080/users/test1
*/
