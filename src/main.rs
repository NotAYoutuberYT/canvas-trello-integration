use crate::canvas::CanvasAPI;
use crate::trello::TrelloAPI;
use std::env;

mod canvas;
mod trello;

#[tokio::main]
async fn main() {
    // build canvas api
    let canvas_api = CanvasAPI::new(
        env::var("CANVAS_ACCESS_TOKEN")
            .expect("Failed to get `CANVAS_ACCESS_TOKEN` environment variable"),
    );

    let trello_api = TrelloAPI::new(
        env::var("TRELLO_API_KEY").expect("Failed to get `TRELLO_API_KEY` environment variable"),
        env::var("TRELLO_API_TOKEN")
            .expect("Failed to get `TRELLO_API_TOKEN` environment variable"),
    );

    println!(
        "{:?}",
        trello_api
            .setup_webhook("https://sample.url/trellowebhook", "sample")
            .await
    );
}
