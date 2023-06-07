use crate::persistent_info::PersistentInfo;
use crate::trello::TrelloBoard;
use actix_web::web::Data;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use std::sync::Mutex;

/// represents the data sent by a webhook tracking a board
#[derive(Deserialize)]
pub struct TrelloBoardResponse {
    model: TrelloBoard,
}

/// handles changes to the board that contains todos
pub async fn todo_board_handler(
    data: Data<Mutex<PersistentInfo>>,
    payload: web::Json<TrelloBoardResponse>,
) -> impl Responder {
    // update the board
    let mut persistent_info = data.lock().expect("Failed to lock persistent info");
    persistent_info.todo_board = payload.model.clone();

    println!(
        "Received change to Todo Board ({})",
        persistent_info.todo_board.name()
    );

    // respond with ok
    HttpResponse::Ok().finish()
}
