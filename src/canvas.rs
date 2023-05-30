/// represents a Canvas course
#[derive(serde::Deserialize)]
pub struct CanvasCourse {
    name: String,
    id: i64,
}

impl CanvasCourse {
    /// returns the course's id
    pub fn id(&self) -> i64 {
        self.id
    }

    /// returns the course's name
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

/// a struct that represents Canvas API information
pub struct CanvasAPI {
    token: String,
    post_client: reqwest::Client,
}

impl CanvasAPI {
    /// creates a new CanvasAPI from a token
    pub fn new(token: String) -> CanvasAPI {
        CanvasAPI {
            token,
            post_client: reqwest::Client::new(),
        }
    }

    /// gets all courses
    pub async fn get_courses(&self) -> Result<Vec<CanvasCourse>, reqwest::Error> {
        // get the url to request to
        let url = format!(
            "https://ames.instructure.com/api/v1/courses?access_token={}",
            self.token
        );

        // get a response from the api and return it
        let response = reqwest::get(&url)
            .await?
            .json::<Vec<CanvasCourse>>()
            .await?;
        Ok(response)
    }
}
