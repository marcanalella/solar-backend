use actix_web::{HttpRequest, HttpResponse, Responder};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}