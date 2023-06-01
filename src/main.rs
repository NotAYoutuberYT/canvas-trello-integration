use crate::canvas::CanvasAPI;
use std::env;

mod canvas;
mod trello;

#[tokio::main]
async fn main() {
    // build canvas api
    let client_api = CanvasAPI::new(
        env::var("CANVAS_ACCESS_TOKEN")
            .expect("Failed to get `CANVAS_ACCESS_TOKEN` environment variable"),
    );

    let todos = client_api.get_todos().await.expect("Unable to get courses");

    println!("\n\tTodos:");
    for todo in todos {
        println!(
            "{}: {}, {}",
            todo.assignment().name(),
            todo.assignment().due_date().unwrap(),
            todo.assignment().url()
        );
    }
}
