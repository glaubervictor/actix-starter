use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::env;

#[get("/healthz")]
async fn healthz() -> impl Responder {
    HttpResponse::Ok().body("Alive")
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/echo2")]
async fn echo2(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/echo3")]
async fn echo3(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/echo4")]
async fn echo4(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/echo10")]
async fn echo1(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").expect("Missing port number");
    let port = port.parse::<u16>().expect("Invalid port given");

    HttpServer::new(|| {
        App::new()
            .service(healthz)
            .service(hello)
            .service(echo)
            .service(echo2)
            .service(echo3)
            .service(echo4)
            .service(echo1)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
