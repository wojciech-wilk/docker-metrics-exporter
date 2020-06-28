use actix_web::{get, Responder};

#[get("/metrics")]
pub async fn metrics() -> impl Responder {
    "Hello World!"
}
