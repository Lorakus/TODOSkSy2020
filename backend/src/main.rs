mod models;
mod config;
mod handlers;
mod db;

use actix_web::{HttpServer, App, web};
use std::io;
use dotenv::dotenv;
use tokio_postgres::NoTls;


use crate::handlers::*;

#[actix_rt::main]
async fn main() -> io::Result<()> {

    dotenv().ok();
    let config =  crate::config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    println!("Started server at {}:{}", config.server.host, config.server.port);

    HttpServer::new(move||{
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(status))
            .route("/todos{_:/?}", web::get().to(get_todos))
            .route("/todos{_:/?}", web::post().to(create_todo))
            .route("/todos/{id}{_:/?}", web::get().to(get_todo_by_id))
            .route("/todos/{id}{_:/?}", web::put().to(edit_todo_by_id))
            .route("/todos/{id}{_:/?}", web::delete().to(delete_todo))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}