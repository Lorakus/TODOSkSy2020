use serde::{Serialize, Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize)]
pub struct Status {
    pub status: String
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "todo_item")]
pub struct TodoItem {
    pub id: i32,
    pub title: String,
    pub procent: i32,
    pub deadline: String,
}

#[derive(Deserialize)]
pub struct CreateTodo{
    pub title: String,
    pub procent: i32,
    pub deadline: String,
}

#[derive(Deserialize)]
pub struct CreateTodoList {
    pub title: String,
}

#[derive(Deserialize)]
pub struct EditTodo{
    pub title: String,
    pub procent: i32,
    pub deadline: String,
}

