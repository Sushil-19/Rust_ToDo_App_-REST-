use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo { // New struct
    pub title: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(title: String) -> Todo {
        Todo {
            id: Uuid::new_v4(),
            title,
            completed: false,
        }
    }
}
