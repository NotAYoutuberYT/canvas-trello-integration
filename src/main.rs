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

    let boards = client_api.get_courses().await.expect("Unable to get boards");
    for board in boards {
        println!("{}", board.name());
    }
}
