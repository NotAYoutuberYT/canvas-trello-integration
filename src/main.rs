use crate::canvas::CanvasAPI;
use crate::trello::{TrelloAPI, TrelloCard};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::env;
use std::future::Future;
use std::pin::Pin;
use reqwest::StatusCode;

mod canvas;
mod trello;

async fn webhook_handler(payload: web::Json<TrelloCard>) -> impl Responder {
    println!("Received webhook event: {:?}", payload);

    // Handle the incoming event according to your application's requirements

    HttpResponse::Ok().body("Sup") // Respond with "OK" to acknowledge receipt of the webhook
}

async fn start_server() {
    let _server =
        HttpServer::new(|| App::new().route("/trellocallbacks", web::post().to(webhook_handler)))
            .bind(("0.0.0.0", 8080))
            .expect("Unable to bind server")
            .run().await;
}

#[actix_web::main]
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

    let mut future = start_server();
    let pinfuture = Box::pin(&mut future);
    pinfuture.poll();

    // check this out: https://stackoverflow.com/questions/65663021/how-to-call-an-async-function-in-poll-method

    println!(
        "{:?}",
        trello_api
            .setup_webhook(
                "http://MYIP:8080/trellocallbacks",
                "6477abd6c96004ac36c58cbe"
            )
            .await
    );
}
