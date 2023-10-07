use actix_web::{get,  web, App, HttpServer, Result, HttpRequest};

//https://actix.rs/docs/request/
use serde::Deserialize;


use reqwest;
#[derive(Deserialize, Debug)]
pub struct Order {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub userId: i64,
}
async fn read_orders() {
    let response = reqwest::get("https://jsonplaceholder.typicode.com/posts/1")
        .await.unwrap().json::<Order>()
        .await;

    match response {
        Ok(order) => {
            println!("Processing order response");
        }
        Err(e) => {
            println!("Orders API response cannot be parsed! {}", e)
        }
    };
}


// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok)))
//     .bind(("127.0.0.1", 8080))?.run().await
// }

// #[actix_web::main]
// async fn main() {
//     HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok))).workers(4);
// }

// Type-safe information extraction
// Path
// Example 1
// #[get("/users/{user_id}/{friedn}")]
// async fn index(path: web::Path<(u32, String)>) -> Result<String> {
//     let (user_id, friend) = path.into_inner();
//     Ok(format!("Welcome {}, user_id {}!", friend, user_id))
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().service(index))
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

// Example 2
// #[derive(Deserialize)]
// struct Info {
//     user_id: u32,
//     friend: String,
// }

// #[get("/users/{user_id}/{friend}")]
// async fn index(info: web::Path<Info>) -> Result<String> {
//     Ok(format!("Welcome {}, user_id {}!", info.friend, info.user_id))
// }

// #[get("/users/{user_id}/{friend}")]
// async fn index(req: HttpRequest) -> Result<String> {
//     let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
//     let user_id: u32 = req.match_info().query("user_id").parse().unwrap();
//     Ok(format!("Welcome {}, user_id {}!", name, user_id))
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().service(index))
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

// Query
#[derive(Deserialize)]
struct  Info {
    username: String,
}

#[get("/")]
async fn index(info: web::Query<Info>) -> String {
    format!("Welcome {}!", info.username)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    read_orders().await;
    HttpServer::new(|| App::new().service(index))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}