use crate::canvas::CanvasAPI;
use std::env;

mod canvas;

#[tokio::main]
async fn main() {
    // build canvas api
    let client_api = CanvasAPI::new(
        env::var("CANVAS_ACCESS_TOKEN")
            .expect("Failed to get `CANVAS_ACCESS_TOKEN` environment variable"),
    );

    let courses = client_api
        .get_courses()
        .await
        .expect("Unable to get boards");

    let todos = client_api.get_todos().await.expect("Unable to get courses");

    println!("\tCourses:");
    for course in courses {
        println!("{}", course.name());
    }

    println!("\n\tTodos:");
    for todo in todos {
        println!("{}: {}", todo.assignment().name(), todo.assignment().url());
    }
}
