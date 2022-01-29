/*
HttpServer 接受应用程序工厂，而非应用程序实例。HttpServer 为每个线程构造一个应用程序实例。因此，必须多次构造应用程序数据。如果你想在不同的线程之间共享数据，应该使用一个可共享的对象，例如 Send + Sync。

web::Data 内部使用 Arc（原子引用计数器）。因此，为了避免创建两个 Arc（原子引用计数器），我们应该在使用 App::app_data() 方法注册数据之前，先行创建数据。

pub struct Data<T: ?Sized>(Arc<T>);

app_data 和 data:
If you want to share data between different threads, a shared object should be used, e.g. Arc. Internally Data type uses Arc so data could be created outside of app factory and clones could be stored via App::app_data() method.

*/
use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {}", counter) // <- response with count
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            // Note: using app_data instead of data
            .app_data(counter.clone()) // <- register the created data
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
