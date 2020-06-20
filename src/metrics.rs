use actix_web::{get, Responder};

#[get("/metrics")]
pub async fn index() -> impl Responder {
    "Hello World!"
}
