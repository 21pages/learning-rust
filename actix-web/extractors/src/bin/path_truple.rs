use actix_web::{get, web, Result};

/*
举例来说，对于注册为 /users/{user_id}/{friend} 路径的资源，有两个变量可以被反序列化：user_id 和 friend。这些变量可以被提取到一个元组（tuple）中（如 Path<(u32, String)>），或者被提取到实现了 serde crate 中的 Deserialize trait 的任何结构中。

提取到元组

*/
/// extract path info from "/users/{user_id}/{friend}" url
/// {user_id} - deserializes to a u32
/// {friend} - deserializes to a String
#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn index(web::Path((user_id, friend)): web::Path<(u32, String)>) -> Result<String> {
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
