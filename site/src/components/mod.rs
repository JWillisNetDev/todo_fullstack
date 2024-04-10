use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, Html, html, use_effect, use_state};

#[function_component(HelloServer)]
pub fn hello_server() -> Html {
    let data = use_state(|| None);
    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let resp = Request::get("/api/todo/list").send().await.unwrap();
                    let result = {
                        if !resp.ok() {
                            Err(format!(
                                "Error fetching data {} ({})",
                                resp.status(),
                                resp.status_text(),
                            ))
                        } else {
                            resp.text().await.map_err(|e| e.to_string())
                        }
                    };
                    data.set(Some(result));
                });
            }
            || {}
        });
    }

    match data.as_ref() {
        None => {
            html! { <div>{"No server response."}</div> }
        }
        Some(Ok(data)) => {
            html! { <div>{"Got server response: "}{data}</div> }
        }
        Some(Err(err)) => {
            html! { <div>{"Error requesting data from server: "}{err}</div>}
        }
    }

}