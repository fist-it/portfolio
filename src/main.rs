use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};



#[get("/")]
async fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("./views/index.html"))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
