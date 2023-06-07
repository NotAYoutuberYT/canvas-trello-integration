use crate::trello::{TrelloAPI, TrelloBoard};

/// holds all data needed in handlers
#[derive(Clone)]
pub struct PersistentInfo {
    pub todo_board: TrelloBoard,
    pub ip: String,
    pub port: u16,
    trello_api: TrelloAPI,
}

impl PersistentInfo {
    /// creates a new PersistentInfo
    pub fn new(
        todo_board: &TrelloBoard,
        trello_api: &TrelloAPI,
        ip: &str,
        port: u16,
    ) -> PersistentInfo {
        PersistentInfo {
            todo_board: todo_board.clone(),
            trello_api: trello_api.clone(),
            ip: ip.to_owned(),
            port,
        }
    }

    /// sets up all initial webhooks
    pub async fn setup_webhooks(&self) -> anyhow::Result<()> {
        self.trello_api
            .setup_webhook(
                format!(
                    "http://{}:{}/trellocallbacks/todo-board",
                    self.ip, self.port
                )
                .as_str(),
                self.todo_board.id().as_str(),
            )
            .await?;

        Ok(())
    }
}
