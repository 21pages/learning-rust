use actix_web::{error, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

/*
actix-web 还提供了其它几种提取器：

Data - 如果需要访问应用程序状态。
HttpRequest - HttpRequest 自身既是提取器，它返回 self，以便于你访问请求。
String - 你可以转换请求的有效负载为 字符串（String） 类型。请参阅文档字符串实例。
bytes::Bytes - 你可以转换请求的有效负载为 Bytes 类型。请参阅文档字符串实例。
Payload - 你可以访问请求的有效负载。请参阅实例。
*/

/*
curl -H "Content-Type: application/json" -X POST  --data '{"username":"sun"}' http://localhost:8080/ -v
*/

/// deserialize `Info` from request's body, max payload size is 4kb
async fn index(info: web::Json<Info>) -> impl Responder {
    format!("Welcome {}!", info.username)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        // json 有效负载的最大值，以及自定义错误处理函数
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });

        App::new().service(
            web::resource("/")
                // change json extractor configuration
                .app_data(json_config)
                .route(web::post().to(index)),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
