use actix_web::{guard, web, App, HttpResponse, HttpServer};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}

/*
ResourceHandler::route() 返回路由（Route）对象。可以使用类似于 builder 的模式来配置路由。可用配置方法如下：

Route::guard() 注册一个新的卫语句（guard），每个路由可以注册任意数量的卫语句（guard）。
Route::method() 注册一个方法作为卫语句（guard），每个路由可以注册任意数量的卫语句（guard）。
Route::to() 为路由注册一个异步 handler 函数，仅能注册一个 handler。通常 handler 注册是最后一个配置操作。

curl -X GET  -H "content-type:application/json" http://localhost:8080/user/sun -v
curl -X PUT  -H "content-type:application/json" http://localhost:8080/user/sun -v
*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/prefix").to(index))
            .service(
                web::resource("/user/{name}")
                    .name("user_detail")
                    .guard(guard::Header("content-type", "application/json")) //过滤
                    .route(web::get().to(|| HttpResponse::Ok()))
                    .route(web::put().to(|| HttpResponse::Ok())),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
