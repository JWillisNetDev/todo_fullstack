use diesel::prelude::*;

#[derive(serde::Serialize, Identifiable, Selectable, Queryable)]
#[diesel(table_name=super::schema::todo)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub is_completed: bool,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name=super::schema::todo)]
pub struct CreateTodo<'a> {
    pub title: &'a str,
}

#[derive(serde::Deserialize, AsChangeset)]
#[diesel(table_name=super::schema::todo)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub is_completed: Option<bool>,
}

pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}