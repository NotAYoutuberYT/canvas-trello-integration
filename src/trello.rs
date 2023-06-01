/// represents a Trello board
#[derive(serde::Deserialize)]
pub struct TrelloBoard {
    id: String,
    name: String,
    url: String,
}

impl TrelloBoard {
    /// returns the board's name
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

/// represents a Trello list
#[derive(serde::Deserialize)]
pub struct TrelloList {
    id: String,
    name: String,
}

impl TrelloList {
    /// returns the list's name
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

/// represents a Trello card
#[derive(serde::Deserialize, Debug)]
pub struct TrelloCard {
    id: String,
    name: String,

    #[serde(rename = "idMembers")]
    members: Vec<String>,
}

impl TrelloCard {
    /// returns the card's name
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// returns the ids of the card's members
    pub fn members(&self) -> Vec<String> {
        self.members.clone()
    }
}

/// represents a Trello account
#[derive(serde::Deserialize, Clone)]
pub struct TrelloMember {
    id: String,
    username: String,
}

impl TrelloMember {
    /// returns the member's id
    pub fn id(&self) -> String {
        self.id.clone()
    }

    /// returns the member's username
    pub fn username(&self) -> String {
        self.username.clone()
    }
}

/// a basic structure that represents a Trello api
pub struct TrelloAPI {
    key: String,
    token: String,
    post_client: reqwest::Client,
}

impl TrelloAPI {
    /// creates a new TrelloAPI from a key and token
    pub fn new(key: String, token: String) -> TrelloAPI {
        TrelloAPI {
            key,
            token,
            post_client: reqwest::Client::new(),
        }
    }

    /// gets all the boards attached to the api key
    pub async fn get_boards(&self) -> Result<Vec<TrelloBoard>, reqwest::Error> {
        // get the url to make the request to
        let url = format!(
            "https://api.trello.com/1/members/me/boards?fields=name,url&key={}&token={}",
            self.key, self.token
        );

        // get a response from the api and return it
        let response = reqwest::get(&url).await?.json::<Vec<TrelloBoard>>().await?;
        Ok(response)
    }

    /// returns the board with the given name, if it exists
    pub async fn get_board(&self, name: &str) -> Result<Option<TrelloBoard>, reqwest::Error> {
        // get all boards
        let boards = self.get_boards().await?;

        // return the first board with the correct name found
        for board in boards {
            if board.name == name {
                return Ok(Some(board));
            }
        }

        // no board with the name found
        Ok(None)
    }

    /// gets all the lists on a board
    pub async fn get_lists_in_board(
        &self,
        board: &TrelloBoard,
    ) -> Result<Vec<TrelloList>, reqwest::Error> {
        // get the url to make the request to
        let url = format!(
            "https://api.trello.com/1/boards/{}/lists?key={}&token={}",
            board.id, self.key, self.token
        );

        // get a response from the api and return it
        let response = reqwest::get(&url).await?.json::<Vec<TrelloList>>().await?;
        Ok(response)
    }

    /// returns the list with the given name, if it exists
    pub async fn get_list_in_board(
        &self,
        board: &TrelloBoard,
        name: &str,
    ) -> Result<Option<TrelloList>, reqwest::Error> {
        // get all boards
        let lists = self.get_lists_in_board(board).await?;

        // return the first board with the correct name found
        for list in lists {
            if list.name == name {
                return Ok(Some(list));
            }
        }

        // no board with the name found
        Ok(None)
    }

    /// gets all cards available to the api
    pub async fn get_cards(&self) -> Result<Vec<TrelloCard>, reqwest::Error> {
        // get the url to make the request to
        let url = format!(
            "https://api.trello.com/1/members/me/cards?key={}&token={}&members=true",
            self.key, self.token
        );

        // get a response from the url and return it
        let response = reqwest::get(&url).await?.json::<Vec<TrelloCard>>().await?;
        Ok(response)
    }

    /// gets all the cards of a list
    pub async fn get_cards_in_list(
        &self,
        list: &TrelloList,
    ) -> Result<Vec<TrelloCard>, reqwest::Error> {
        // get the url to make the request to
        let url = format!(
            "https://api.trello.com/1/lists/{}/cards?key={}&token={}&members=true",
            list.id, self.key, self.token
        );

        // get a response from the url and return it
        let response = reqwest::get(&url).await?.json::<Vec<TrelloCard>>().await?;
        Ok(response)
    }

    /// adds an item to a list
    pub async fn add_card_to_list(
        &self,
        list: &TrelloList,
        name: &str,
    ) -> Result<(), reqwest::Error> {
        // get the url to post to
        let url = format!(
            "https://api.trello.com/1/cards?key={}&token={}&idList={}&name={}",
            self.key, self.token, list.id, name
        );

        // make the post
        self.post_client.post(&url).send().await?;
        Ok(())
    }

    /// turns an id into a Trello user
    pub async fn get_user(&self, id: &str) -> Result<TrelloMember, reqwest::Error> {
        // get the url to make the request to
        let url = format!(
            "https://api.trello.com/1/members/{}?key={}&token={}",
            id, self.key, self.token
        );

        let response = reqwest::get(&url).await?.json::<TrelloMember>().await?;
        Ok(response)
    }
}
