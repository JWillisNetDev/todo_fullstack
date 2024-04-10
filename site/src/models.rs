
#[derive(serde::Deserialize, Clone, PartialEq)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub is_completed: bool,
}

#[derive(serde::Serialize)]
pub struct CreateTodoDto {
    pub title: String,
}