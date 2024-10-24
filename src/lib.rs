use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use std::net::TcpListener;

async fn health_check(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct Formdata {
    email: String,
    name: String,
}

async fn subscribe(_form: web::Form<Formdata>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
        })
        // .bind(address)?
        .listen(listener)?
        .run();
    // .await > 대기하지 않도록 제거.
    Ok(server)
}