use std::sync::Mutex;

use actix_web::{get, HttpResponse, Responder, App, post, HttpServer, web};

// Getting Started
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// Application
struct AppState {
    app_name: String,
    counter: Mutex<i32>,
}

async fn index_state(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}

async fn index_mutable_state(data: web::Data<AppState>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request number: {counter}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Note: web::Data created _outside_ HttpServer::new closure
    let counter = web::Data::new(AppState {
        app_name: String::from("Actix Web"),
        counter: Mutex::new(0)
    });

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/state", web::get().to(index_state))
            .route("/mutable_state", web::get().to(index_mutable_state))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
