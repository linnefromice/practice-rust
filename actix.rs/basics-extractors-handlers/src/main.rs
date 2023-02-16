use actix_web::{HttpServer, App, web};
use serde::Deserialize;

#[derive(Deserialize)]
struct UsersInfo {
    user_id: u32,
    friend: String
}
#[actix_web::get("/users/{user_id}/{friend}")]
async fn users(info: web::Path<UsersInfo>) -> std::io::Result<String> {
    Ok(format!("Welcome {}, user_id {}!", info.friend, info.user_id))
}

#[derive(Deserialize)]
struct IndexInfo {
    username: String,
}
#[actix_web::get("/")]
async fn index(info: web::Query<IndexInfo>) -> String {
    format!("Welcome {}!", info.username)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
            .service(index)
            .service(users))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}