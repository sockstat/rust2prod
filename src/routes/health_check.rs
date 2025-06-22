use actix_web::{HttpResponse, Responder, web};

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
