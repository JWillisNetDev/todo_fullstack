use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::wasm_bindgen::JsCast;
use yew::function_component;
use yew::prelude::*;
use crate::models::Todo;
use crate::services::{create_todo, delete_todo, get_todos};

#[derive(Properties, PartialEq)]
pub struct TodoProps {
    pub todo: Todo,
    pub on_deleted: Callback<i32>,
}

#[function_component(TodoItem)]
pub fn todo_item(props: &TodoProps) -> Html {
    let todo = props.todo.clone();
    let on_deleted = props.on_deleted.clone();
    html! {
        <div key={todo.id} class="flex mb-4 items-center bg-green p-2 rounded text-white">
            <input class="mr-4 leading-tight" type="checkbox" checked={todo.is_completed} />
            <p class="w-full font-alfa">{&todo.title}</p>
            <button class="bg-purple hover:bg-purple-light text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline" type="button" onclick={move |_| on_deleted.emit(todo.id)}>
            {"Delete"}
            </button>
        </div>
    }
}

#[function_component(TodoList)]
pub fn todo_list() -> Html {
    let data = use_state(|| None);

    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let todos = get_todos().await;
                    data.set(Some(todos));
                })
            }
        });
    }

    let on_create_new_clicked = {
        let data = data.clone();
        Callback::from(move |_| {
            spawn_local(async move {
                _ = create_todo("New Todo".into()).await.unwrap();
            });
            data.set(None);
        })
    };

    let on_deleted = {
        let data = data.clone();
        Callback::from(move |id: i32| {
            spawn_local(async move {
                delete_todo(id).await.unwrap();
            });
            data.set(None);
        })
    };

    match data.as_ref() {
        None => html! { <p>{"No server response..."}</p> },
        Some(Ok(data)) => {
            html! {
                <div class="h-100 p-4 w-full flex flex-col items-center justify-center bg-green-light rounded shadow font-sans">
                    <div class="p-6 w-full">
                    {
                        data.iter().map(|todo| {
                            html! { <TodoItem todo={todo.clone()} on_deleted={on_deleted.clone()} /> }
                        }).collect::<Html>()
                    }
                    </div>
                    <div class="flex mt-4">
                        <button class="flex-no-shrink p-2 border-2 rounded text-white font-bold bg-green-dark hover:bg-green border-green" onclick={on_create_new_clicked.clone()}>
                        {"Create New"}
                        </button>
                    </div>
                </div>
            }
        },
        Some(Err(err)) => html! {
            <p>{"An error occurred! "}{err}</p>
        }
    }
}