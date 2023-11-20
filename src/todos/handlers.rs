use askama::Template;
use askama_axum::Response;
use axum::extract::State;
use axum::{
    extract::Path,
    response::{Html, IntoResponse},
    Form,
};
use uuid::Uuid;

use crate::auth::models::AuthUser;
use crate::state::AppState;
use crate::todos::models::{CreateTodo, CreateTodoResponse, Index, Todo};

pub async fn handle_index(State(state): State<AppState>, auth_user: AuthUser) -> Response {
    let todos =
        sqlx::query_as::<_, Todo>("SELECT id, task, completed FROM todos WHERE user_id = ?")
            .bind(auth_user.id)
            .fetch_all(&state.pool)
            .await
            .unwrap();

    let template = Index { todos };
    Html(template.render().unwrap()).into_response()
}

pub async fn handle_create(
    State(AppState { pool, .. }): State<AppState>,
    auth_user: AuthUser,
    Form(todo): Form<CreateTodo>
) -> Response {
    let id = Uuid::new_v4();
    sqlx::query("INSERT INTO todos (id, user_id, task, completed) VALUES (?, ?, ?, ?)")
        .bind(id)
        .bind(auth_user.id)
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
        },
    };
    Html(template.render().unwrap()).into_response()
}

pub async fn handle_delete(
    State(AppState { pool, .. }): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>
) -> Response {
    sqlx::query("DELETE FROM todos WHERE id = ? and user_id = ?")
        .bind(id)
        .bind(auth_user.id)
        .execute(&pool)
        .await
        .unwrap();
    Html("").into_response()
}

pub async fn handle_complete(
    State(AppState { pool, .. }): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>
) -> Response {
    sqlx::query("UPDATE todos SET completed = ? WHERE id = ? and user_id = ?")
        .bind(id)
        .bind(auth_user.id)
        .execute(&pool)
        .await
        .unwrap();

    let todo = sqlx::query_as::<_, Todo>(
        "SELECT id, task, completed FROM todos WHERE id = ? and user_id = ?",
    )
    .bind(id)
    .bind(auth_user.id)
    .fetch_one(&pool)
    .await
    .unwrap();

    let template = CreateTodoResponse { todo };
    Html(template.render().unwrap()).into_response()
}
