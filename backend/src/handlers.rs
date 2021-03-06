use crate::db;
use crate::models::{CreateTodo, EditTodo, Status};
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "OK".to_string(),
    })
}

//get list of todos
pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error geting todo's from DB");
    let result = db::get_todos(&client).await;

    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

//create new TODO
pub async fn create_todo(db_pool: web::Data<Pool>, json: web::Json<CreateTodo>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error geting todo's from DB");

    let result = db::create_todo(
        &client,
        json.title.clone(),
        json.procent.clone(),
        json.deadline.clone(),
    )
    .await;

    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

//get todo by id
pub async fn get_todo_by_id(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error geting todo's from DB");
    let result = db::get_todo_by_id(&client, path.0).await;

    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

//edit todo -> id doesnt send succesful message back.....
pub async fn edit_todo_by_id(
    db_pool: web::Data<Pool>,
    path: web::Path<(i32,)>,
    json: web::Json<EditTodo>,
) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error geting todo's from DB");
    let result = db::edit_todo_by_id(
        &client,
        path.0,
        json.title.clone(),
        json.procent.clone(),
        json.deadline.clone(),
    )
    .await;

    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

//delete TODO
pub async fn delete_todo(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error geting todo's from DB");
    let result = db::delete_todo(&client, path.0).await;

    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
