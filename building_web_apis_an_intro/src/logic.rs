use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoItem {
    pub title: String,
    pub notes: String,
    pub assigned_to: String,
    pub completed: bool,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateTodoItem {
    pub title: Option<String>,
    pub notes: Option<String>,
    pub assigned_to: Option<String>,
    pub completed: Option<bool>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndentifyableTodoItem {
    pub id: usize,

    #[serde(flatten)]
    pub item: TodoItem,
}