use axum::{
    extract::{Path, State},
    http::{ header:: HeaderMap, StatusCode},
    response::{sse::Event, Html, IntoResponse, Response, Sse},
    Extension, 
    Form,
    Json,
};


use yew::ServerRenderer;

use serde_json::json;
use std::convert::Infallible;
use std::time::Duration;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::{Stream, StreamExt as _};


use crate::models::{MutationKind, Todo, TodoNew, TodoUpdate};
use crate::{errors::ApiError, router::AppState, router::TodosStream, templates};


pub async fn home() -> impl IntoResponse {
    templates::HelloTemplate
}

pub async fn stream() -> impl IntoResponse {
    templates::StreamTemplate
}

pub async fn fetch_todos(
    headers: HeaderMap,
    State(state): State<AppState>,  
) -> Result<impl IntoResponse, ApiError> {
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM TODOS")
        .fetch_all(&state.db)
        .await?;

    match headers.get("HX-Request").is_some() {
        true => {
            let props = templates::RecordsProps{
                todos: todos
            };
            let rendered = ServerRenderer::<templates::CreateTodos>::with_props(|| props)
                .render()
                .await;

            return Ok(Html(rendered).into_response())
        }
        _ =>  {
            return Ok(Json(todos).into_response())
        } 
    }

}

pub async fn styles() -> Result<impl IntoResponse, ApiError> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/css")
        .body(include_str!("../templates/styles.css").to_owned())?;

    Ok(response)
}

pub async fn create_todo(
    headers: HeaderMap,
    State(state): State<AppState>,
    Extension(tx): Extension<TodosStream>,
    Form(form): Form<TodoNew>,
) -> Result<impl IntoResponse, ApiError> {
    let todo = sqlx::query_as::<_, Todo>(
        "INSERT INTO TODOS (description) VALUES ($1) RETURNING id, description",
    )
    .bind(form.description)
    .fetch_one(&state.db)
    .await
    .unwrap();

    if tx
        .send(TodoUpdate {
            mutation_kind: MutationKind::Create,
            id: todo.id,
        })
        .is_err() 
    {
        eprintln!(
            "Record with ID {} was deleted but nobody's listening to the stream!",
            todo.id
        );
    }
    let props = templates::TodoProps{
        todo: todo
    };
    match headers.get("HX-Request").is_some() {
        true => {
            let rendered = ServerRenderer::<templates::CreateTodo>::with_props(|| props)
                .render()
                .await;
            return Ok(Html(rendered).into_response());
        }
        _ =>  {
            return Ok(Json(props).into_response());
        } 
    }
}
pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Extension(tx): Extension<TodosStream>,
) -> Result<impl IntoResponse, ApiError> {
    sqlx::query("DELETE FROM TODOS WHERE ID = $1")
        .bind(id)
        .execute(&state.db)
        .await?;

    if tx
        .send(TodoUpdate {
            mutation_kind: MutationKind::Delete,
            id,
        })
        .is_err()
    {
        eprintln!(
            "Record with ID {} was deleted but nobody's listening to the stream!",
            id
        );
    }

    Ok(StatusCode::OK)
}

pub async fn handle_stream(
    Extension(tx): Extension<TodosStream>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = tx.subscribe();

    let stream = BroadcastStream::new(rx);

    Sse::new(
        stream
            .map(|msg| {
                let msg = msg.unwrap();
                let json = format!("<div>{}</div>", json!(msg));
                Event::default().data(json)
            })
            .map(Ok),
    )
    .keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(600))
            .text("keep-alive-text"),
    )
}
