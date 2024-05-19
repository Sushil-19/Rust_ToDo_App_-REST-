# Rust_ToDo_App_-REST-

This is a Rest Application.

Run Application : cargo run

Create a new ToDo : curl -X POST -H "Content-Type: application/json" -d '{"title":"Learn Rust"}' http://127.0.0.1:8080/todos

Get all ToDos : curl http://127.0.0.1:8080/todos

Get a specific ToDo by ID : curl http://127.0.0.1:8080/todos/{id}

Update a ToDo : curl -X PUT -H "Content-Type: application/json" -d '{"title":"Learn Rust", "completed":true}' http://127.0.0.1:8080/todos/{id}

Delete a ToDo : curl -X DELETE http://127.0.0.1:8080/todos/{id}
