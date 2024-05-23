
use serde::{Deserialize, Serialize};
use yew::prelude::*;


use crate::models;

#[derive(Template)]
#[template(path = "stream.html")]
pub struct StreamTemplate;

#[derive(yew::Properties, PartialEq, Serialize, Deserialize)]
pub struct RecordsProps {
    pub todos: Vec<models::Todo>,
}

#[derive(yew::Properties, PartialEq, Serialize, Deserialize)]
pub struct TodoProps {
    pub todo: models::Todo,
}

#[derive(yew::Properties, PartialEq, Serialize, Deserialize)]
pub struct AppProps;

#[function_component(App)]
pub fn app(props: &AppProps) -> Html {
    html! {
            <html lang="en">
            <doctype html=true>
                <head>
                    <script
                        src="https://unpkg.com/htmx.org@1.9.6"
                        integrity="sha384-FhXw7b6AlE/jyjlZH5iHa/tTe9EpJ1Y55RjcgPbjeWMskSxZt1v9qkxLJWNJaGni"
                        crossorigin="anonymous"
                    ></script>
                    <link rel="stylesheet" href="/styles.css" />
                    <title>{"Todo"}</title>
                </head>
                <body>
                    <div id="content">
                        <h1>{"Shuttle Todos - Home"}</h1>
                        <a href="/stream">{"Event stream"}</a>
                        <h1>{"Shuttle Todos"}</h1>
                        <form id="add-form">
                            <input
                                placeholder="Your todo description..."
                                required=true
                                type="text"
                                name="description"
                            />
                            <button
                                hx-post="/todos"
                                hx-trigger="click"
                                hx-target="#todos-content"
                                hx-swap="beforeend"
                            >
                                {"Add"}
                            </button>
                        </form>
                        <div
                            id="list"
                            hx-get="/todos"
                            hx-target="this"
                            hx-trigger="load"
                            hx-swap="outerHTML"
                        >
                            {"Loading..."}
                        </div>
                    </div>
                </body>
            </html>
    }
}


#[function_component(CreateTodo)]
pub fn create_todo(props: &TodoProps) -> Html {
    let value = html! {
        <tr id={format!("shuttle-todo-{}", props.todo.id)}>
            <td>{ props.todo.id }</td>
            <td id={format!("shuttle-todo-desc-{}" , props.todo.id)}>{ &props.todo.description }</td>
            <td>
                <button
                    hx-delete={format!("/todos/{}", props.todo.id)}
                    hx-trigger="click"
                    hx-target={format!("#shuttle-todo-{}", props.todo.id)}
                    hx-swap="delete"
                >
                {"Delete"}
                </button>
            </td>
        </tr>
    };
    return value
}

#[function_component(CreateTodos)]
pub fn create_todos(props: &RecordsProps) -> Html {
    let value = html! {
        <div id="todos">
            <table>
                <thead>
                    <tr>
                        <th>{"ID"}</th>
                        <th>{"Description"}</th>
                        <th>{"Delete"}</th>
                    </tr>
                </thead>
                <tbody id="todos-content">
                    {for props.todos.iter().map(|todo| html_nested!{ 
                        <CreateTodo todo={todo.clone()}/> 
                    })}
                </tbody>
            </table>
        </div>
    };

    return value
}


