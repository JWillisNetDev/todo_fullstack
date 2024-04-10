use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::HelloServer;
use crate::pages::TodoList;

mod components;
mod pages;
mod services;
mod models;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/todos")]
    Todos,
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {
            <div class="bg-green">
                <h1 class="p-4 text-3xl font-alfa text-white">{"Hello, world! You're on Rust!"}</h1>
            </div>
        },
        Route::Todos => html! { <TodoList /> }
    }
}
