/*
实现了Responder trait的外置类型:
&'a String, &'static [u8], &'static str, (T, StatusCode), Option<T>, Result<T,E>, String
*/

use actix_web::{self, get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
// use bytes::Bytes;
// use futures::Future;

// async fn index(_req: HttpRequest) -> impl Responder {
//     HttpResponse::Ok().body(Bytes::from_static(b"Hello world!").bytes())
// }

// async fn index2(req: HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {}
async fn index3(_req: HttpRequest) -> &'static str {
    "Hello world 3 !"
}

async fn index4(_req: HttpRequest) -> String {
    "Hello world 4 !".to_owned()
}

#[get("/")]
async fn index5(_req: HttpRequest) -> Option<String> {
    Some("hello".to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/3", web::get().to(index3))
            .service(index5)
            .route("/4", web::get().to(index4))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
