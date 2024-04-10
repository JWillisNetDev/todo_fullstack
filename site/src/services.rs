use gloo_net::http::Request;
use crate::models::{Todo, CreateTodoDto};

pub async fn get_todos() -> Result<Vec<Todo>, String> {
    let resp = Request::get("/api/todo/list").send().await.unwrap();
    if !resp.ok() {
        Err(format!(
            "Error getting todos {} ({})",
            resp.status(),
            resp.status_text()
        ))
    } else {
        let resp = resp.text().await.map_err(|e| e.to_string())?;
        serde_json::from_str::<Vec<Todo>>(resp.as_str()).map_err(|e| e.to_string())
    }
}

pub async fn create_todo(title: String) -> Result<Todo, String> {
    let item = CreateTodoDto { title };
    let resp = Request::post("/api/todo/create")
        .body(item.title).unwrap()
        .send()
        .await
        .unwrap();
    if !resp.ok() {
        Err(format!{
            "Error creating todo {} ({})",
            resp.status(),
            resp.status_text(),
        })
    } else {
        let resp = resp.text().await.map_err(|e| e.to_string())?;
        serde_json::from_str::<Todo>(resp.as_str()).map_err(|e| e.to_string())
    }
}

pub async fn delete_todo(id: i32) -> Result<(), String> {
    let resp = Request::delete(format!("/api/todo/{id}").as_str())
        .send()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}