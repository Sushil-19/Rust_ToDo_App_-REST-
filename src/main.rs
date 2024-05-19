use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use uuid::Uuid;

mod handlers;
mod models;

use handlers::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let todo_list = web::Data::new(Mutex::<Vec<models::Todo>>::new(vec![])); // specify the type

    HttpServer::new(move || {
        App::new()
            .app_data(todo_list.clone())
            .route("/todos", web::get().to(get_todos))
            .route("/todos/{id}", web::get().to(get_todo_by_id))
            .route("/todos", web::post().to(create_todo))
            .route("/todos/{id}", web::put().to(update_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
