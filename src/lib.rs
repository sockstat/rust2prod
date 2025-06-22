use actix_web::web::Form;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use serde::{Deserialize, Serialize};

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

fn index(form: web::Form<FormData>) -> String {
    format!("Welcome {}", form.name)
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .bind("127.0.0.1:9000")?
    .run();

    Ok(server)
}
