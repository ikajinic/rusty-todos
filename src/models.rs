use askama::Template;
use serde::Deserialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateTodo {
    pub task: String,
}

#[derive(FromRow)]
pub struct Todo {
    pub id: Uuid,
    pub task: String,
    pub completed: bool,
}

#[derive(Template)]
#[template(path = "todo-item.html")]
pub struct CreateTodoResponse {
    pub todo: Todo,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    pub todos: Vec<Todo>,
}
