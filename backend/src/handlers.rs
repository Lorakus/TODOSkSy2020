
use crate::models::Status;
use actix_web::{Responder, web};


pub async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status {status: "OK".to_string()})
}