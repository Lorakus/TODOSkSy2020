use crate::models::{Status, CreateTodo};
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

pub async fn create_todo(db_pool: web::Data<Pool>, json: web::Json<CreateTodo>) -> impl Responder{


    let client: Client = 
        db_pool.get().await.expect("Error geting todo's from DB");

    let result = db::create_todo(&client,json.title.clone(), json.procent.clone(), json.deadline.clone()).await;

    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}


