mod models;
mod config;

use crate::models::Status;
use actix_web::{Responder, HttpServer, App, web};
use std::io;
use dotenv::dotenv;
use tokio_postgres::NoTls;

async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status {status: "OK".to_string()})
}


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

    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}