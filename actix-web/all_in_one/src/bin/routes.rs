use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

async fn route_2() -> impl Responder {
    "2"
}

#[get("/3")]
async fn factory_3(_req: HttpRequest) -> impl Responder {
    Some("3".to_string())
}

async fn service_scope_4() -> impl Responder {
    "4".to_string()
}

// this function could be located in a different module
fn config_6(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/6")
            .route(web::get().to(|| HttpResponse::Ok().body("6")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

struct AppState {
    app_name: String,
}

#[get("/7")]
async fn data_7(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name

    format!("{}", app_name) // <- response with app_name
}

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

#[get("/8")]
async fn share_data_8(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("8.Request number: {}", counter) // <- response with count
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .route("/1", web::get().to(|| HttpResponse::Ok().body("1")))
            .route("/2", web::get().to(route_2))
            .service(factory_3)
            .service(web::scope("/4").route("/sub1", web::get().to(service_scope_4)))
            .service(web::resource("/5").to(|| HttpResponse::Ok().body("5")))
            .configure(config_6)
            .data(AppState {
                app_name: String::from("7"),
            })
            .service(data_7)
            .app_data(counter.clone())
            .service(share_data_8)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
