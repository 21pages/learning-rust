use actix_web::{get, middleware, App, HttpResponse, HttpServer};

/*
actix-web 可以使用 Compress 中间价自动压缩 有效负载。支持以下编解码器：

Brotli
Gzip
Deflate
Identity

*/

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("plain/text")
        .header("X-Hdr", "sample")
        .body("data")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default()) //encode
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
