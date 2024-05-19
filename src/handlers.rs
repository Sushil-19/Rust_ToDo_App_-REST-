use actix_web::{web, HttpResponse};
use serde_json::json;
use std::sync::Mutex;
use uuid::Uuid;

use crate::models::{CreateTodo, Todo};

type TodoList = Mutex<Vec<Todo>>;

pub async fn get_todos(data: web::Data<TodoList>) -> HttpResponse {
    let todos = data.lock().unwrap();
    HttpResponse::Ok().json(&*todos) // dereference here
}

pub async fn get_todo_by_id(data: web::Data<TodoList>, id: web::Path<Uuid>) -> HttpResponse {
    let todos = data.lock().unwrap();
    if let Some(todo) = todos.iter().find(|todo| todo.id == *id) {
        HttpResponse::Ok().json(todo)
    } else {
        HttpResponse::NotFound().json(json!({"error": "Todo not found"}))
    }
}

pub async fn create_todo(
    data: web::Data<TodoList>,
    new_todo: web::Json<CreateTodo>,
) -> HttpResponse {
    let mut todos = data.lock().unwrap();
    let todo = Todo::new(new_todo.title.clone());
    todos.push(todo);
    HttpResponse::Created().json(&*todos) // dereference here
}

pub async fn update_todo(
    data: web::Data<TodoList>,
    id: web::Path<Uuid>,
    updated_todo: web::Json<Todo>,
) -> HttpResponse {
    let mut todos = data.lock().unwrap();
    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == *id) {
        todo.title = updated_todo.title.clone();
        todo.completed = updated_todo.completed;
        HttpResponse::Ok().json(todo)
    } else {
        HttpResponse::NotFound().json(json!({"error": "Todo not found"}))
    }
}

pub async fn delete_todo(
    data: web::Data<TodoList>,
    id: web::Path<Uuid>,
) -> HttpResponse {
    let mut todos = data.lock().unwrap();
    if let Some(pos) = todos.iter().position(|todo| todo.id == *id) {
        todos.remove(pos);
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().json(json!({"error": "Todo not found"}))
    }
}
