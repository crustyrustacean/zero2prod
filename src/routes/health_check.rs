// src/routes/health_check.rs

// dependencies
use actix_web::HttpResponse;

// health_check route endpoint handler
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
