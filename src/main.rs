use actix_web::{get, App, HttpRequest, HttpServer, Responder};
use actix_files as fs;
use fs::{NamedFile, Files};

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    NamedFile::open_async("./views/index.html").await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(Files::new("/css", "./views/static/css"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
