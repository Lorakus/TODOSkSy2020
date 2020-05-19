
use crate::models::Status;
use crate::db;
use actix_web::{Responder, web, HttpResponse};
use deadpool_postgres::{Pool, Client};


pub async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status {status: "OK".to_string()})
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder{

    let client: Client = 
        db_pool.get().await.expect("Error geting todo's from DB");
    let result = db::get_todos(&client).await;

    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into()
    }

}