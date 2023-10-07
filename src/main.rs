use actix_web::{web, App, HttpServer, HttpResponse};

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok)))
//     .bind(("127.0.0.1", 8080))?.run().await
// }

#[actix_web::main]
async fn main() {
    HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok))).workers(4);
}