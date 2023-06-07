use crate::canvas::CanvasAPI;
use crate::trello::{TrelloAPI, TrelloCard};
use actix_web::{rt, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Context;
use reqwest::StatusCode;
use std::time::Duration;
use std::{env, thread};

mod canvas;
mod trello;

async fn webhook_handler() -> impl Responder {
    println!("Received webhook event");

    // Handle the incoming event according to your application's requirements

    HttpResponse::Ok().finish() // Respond with "OK" to acknowledge receipt of the webhook
}

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

    // create a server
    println!("Starting HTTP server...");
    let server = HttpServer::new(|| App::new().route("/trellocallbacks", web::post().to(webhook_handler)).route("/trellocallbacks", web::head().to(HttpResponse::Ok)))
        .workers(5)
        .bind(("0.0.0.0", port))
        .with_context(|| {
            "Unable to bind server (probably due to incorrect PUBLIC_IP or TRELLO_CANVAS_PORT environment variables)"
        })?
        .run();
    let server_handle = server.handle();
    rt::spawn(server);

    // verify server is running properly
    println!("Ensuring server is running... (if this takes longer than a few seconds, confirm the TRELLO_CANVAS_PORT and PUBLIC_IP environment variables are correct)");
    reqwest::get(format!("http://localhost:{port}/trellocallbacks/"))
        .await
        .with_context(|| "Failed to validate HTTP server")?;
    println!("Server verified! Beginning process...\n\n\tLog:");

    // debug atm, prints an attempt to start a webhook
    println!(
        "{:?}",
        trello_api
            .setup_webhook(
                format!("http://{ip}:{port}/trellocallbacks").as_str(),
                "6477abd6c96004ac36c58cbe"
            )
            .await
    );

    // stop server
    thread::sleep(Duration::from_secs(15));
    server_handle.stop(true).await;

    Ok(())
}
