use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok() // .finish() 생략 가능.
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/health_check", web::get().to(health_check)) // route 경유해 App에 등록.
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
