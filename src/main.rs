use crate::canvas::CanvasAPI;
use crate::persistent_info::PersistentInfo;
use crate::trello::TrelloAPI;
use crate::web_handlers::todo_board_handler;
use actix_web::web::Data;
use actix_web::{rt, web, App, HttpResponse, HttpServer};
use anyhow::Context;
use std::env;
use std::io::Read;
use std::sync::Mutex;
use tokio::net::windows::named_pipe::PipeEnd::Client;

mod canvas;
mod persistent_info;
mod trello;
mod web_handlers;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // build canvas api
    let canvas_api = CanvasAPI::new(
        env::var("CANVAS_ACCESS_TOKEN")
            .expect("Failed to get `CANVAS_ACCESS_TOKEN` environment variable"),
    );

    // build trello api
    let trello_api = TrelloAPI::new(
        env::var("TRELLO_API_KEY").expect("Failed to get `TRELLO_API_KEY` environment variable"),
        env::var("TRELLO_API_TOKEN")
            .expect("Failed to get `TRELLO_API_TOKEN` environment variable"),
    );

    // get server port
    let port: u16 = env::var("TRELLO_CANVAS_PORT")
        .expect("Failed to get `TRELLO_CANVAS_PORT` environment variable")
        .parse()
        .expect("unable to parse port environment variable");

    // get server ip
    let ip = env::var("PUBLIC_IP").expect("Failed to get `PUBLIC_IP` environment variable");

    // setup persistent data
    println!("Getting Trello information...");
    let todo_board = trello_api.get_board("To-Dos").await.unwrap().unwrap();
    let persistent_info = PersistentInfo::new(&todo_board, &trello_api, ip.as_str(), port);
    let data = Data::new(Mutex::new(persistent_info.clone()));

    // create a server
    println!("Starting HTTP server...");

    let data_clone = data.clone();
    let server = HttpServer::new(move || App::new()
            .app_data(Data::clone(&data_clone))
            .route("/trellocallbacks/todo-board", web::post().to(todo_board_handler))
            .route("/trellocallbacks/todo-board", web::head().to(HttpResponse::Ok))
            .route("/", web::head().to(HttpResponse::Ok)))
        .workers(4)
        .bind(("0.0.0.0", port))
        .with_context(|| {
            "Unable to bind server (probably due to incorrect PUBLIC_IP or TRELLO_CANVAS_PORT environment variables)"
        })?
        .run();
    let server_handle = server.handle();
    rt::spawn(server);

    // verify server is running properly
    println!("Ensuring server is running... (if this takes longer than a few seconds, confirm the TRELLO_CANVAS_PORT and PUBLIC_IP environment variables are correct)");
    let temp_client = reqwest::Client::new();
    let response = temp_client
        .head(format!("http://localhost:{port}/"))
        .send()
        .await
        .with_context(|| "Failed to validate HTTP server")?;
    if !response.status().is_success() {
        return Err(anyhow::Error::msg(
            "Local server gave error when validating, terminating process...",
        ));
    }

    // set up webhooks
    println!("Setting up initial webhooks...");
    persistent_info.setup_webhooks().await?;
    drop(persistent_info);

    // wait until input is given
    println!("Ready to begin, press enter to exit...\n\n\tLog:");
    let _ = std::io::stdin()
        .read(&mut [0u8])
        .expect("Failed to read from stdin for interrupt");

    // stop server
    println!("Stopping server...");
    server_handle.stop(true).await;

    Ok(())
}
