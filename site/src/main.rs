use yew::prelude::*;
use yew_router::prelude::*;
use components::HelloServer;

mod components;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/test")]
    HelloServer,
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
        Route::Home => html! { <h1 class="text-green text-3xl m-4 font-bold underline">{"Hello, world! You're on Rust!"}</h1> },
        Route::HelloServer => html! { <HelloServer /> }
    }
}
