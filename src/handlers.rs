use askama::Template;
use askama_axum::Response;
use axum::{extract::Path, Form, response::{Html, IntoResponse}};
use axum::extract::State;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::{CreateTodo, CreateTodoResponse, Index, Todo};

pub async fn handle_index(State(pool): State<SqlitePool>) -> Response {
    let todos = sqlx::query_as::<_, Todo>("SELECT id, task, completed FROM todos")
        .fetch_all(&pool)
        .await
        .unwrap();

    let template = Index {
        todos,
    };
    Html(template.render().unwrap()).into_response()
}

pub async fn handle_create(State(pool): State<SqlitePool>, Form(todo): Form<CreateTodo>) -> Response {
    let id = Uuid::new_v4();
    sqlx::query("INSERT INTO todos (id, task, completed) VALUES (?, ?, ?)")
        .bind(id)
        .bind(&todo.task)
        .bind(false)
        .execute(&pool)
        .await
        .unwrap();

    let template = CreateTodoResponse {
        todo: Todo {
            id,
            task: todo.task.to_owned(),
            completed: false,
        }
    };
    Html(template.render().unwrap()).into_response()
}

pub async fn handle_delete(Path(id): Path<Uuid>, State(pool): State<SqlitePool>) -> Response {
     sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();
    Html("").into_response()
}

pub async fn handle_complete(Path(id): Path<Uuid>, State(pool): State<SqlitePool>) -> Response {
    sqlx::query("UPDATE todos SET completed = ? WHERE id = ?")
        .bind(true)
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();

    let todo = sqlx::query_as::<_, Todo>("SELECT id, task, completed FROM todos WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();

    let template = CreateTodoResponse {
        todo
    };
    Html(template.render().unwrap()).into_response()
}
