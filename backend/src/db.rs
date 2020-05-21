use crate::models::{TodoItem};
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoItem>, io::Error>{

    let statement = client.prepare("select * from todo_item order by id desc").await.unwrap();

    let todos = client.query(&statement, &[])
        .await
        .expect("ERROR GETTING TODO")
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>();
    Ok(todos)
}

pub async fn create_todo(client: &Client, title: String, procent: i32, deadline: String) -> Result<TodoItem, io::Error>{

    let statement = client.prepare("insert into todo_item (title, procent, deadline) values ($1, $2, $3) returning id, title, procent, deadline").await.unwrap();
    
    client.query(&statement,&[&title, &procent, &deadline])
        .await
        .expect("error creating new todo")
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>()
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "error creatign new todo"))
}


pub async fn get_todo_by_id(client: &Client, id: i32) -> Result<Vec<TodoItem>, io::Error> {
    let statement = client.prepare("select * from todo_item where id = $1").await.unwrap();

    let item = client.query(&statement, &[&id])
        .await
        .expect("ERROR GETTING TODO")
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>();
    Ok(item)
}

pub async fn delete_todo(client: &Client, id: i32) -> Result<Vec<TodoItem>, io::Error> {
    let statement = client.prepare("delete from todo_item where id = $1").await.unwrap();

    let item = client.query(&statement, &[&id])
        .await
        .expect("ERROR GETTING TODO")
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>();
    Ok(item)
}

pub async fn edit_todo_by_id(client: &Client, id: i32, title: String, procent: i32, deadline: String) -> Result<Vec<TodoItem>, io::Error> {
    let statement = client.prepare("update todo_item set title = $2,  procent = $3, deadline = $4 where id = $1").await.unwrap();

    let item = client.query(&statement, &[&id, &title, &procent, &deadline])
        .await
        .expect("ERROR GETTING TODO")
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>();
    Ok(item)
} 