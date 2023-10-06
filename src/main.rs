use std::sync::Mutex;

use actix_web::{cookie::time::Weekday, get, post, web, App, HttpResponse, HttpServer, Responder};

struct AppState {
    app_name: String,
    counter: Mutex<i32>,
}

#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    HttpResponse::Ok().body(format!("Hello {} -> {}!", data.app_name, counter))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix-web"),
                counter: Mutex::new(0),
            }))
            .service(hello)
            .service(web::scope("/users")
            .route(
                "/get",
                web::to(|| async { HttpResponse::Ok().body("users") }),
            ))
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
