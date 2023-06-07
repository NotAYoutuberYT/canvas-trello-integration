use chrono::{DateTime, Local};
use url::Url;

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

/// a deserializable Canvas assignment without proper representation of data
#[derive(serde::Deserialize, Clone)]
struct PartialCanvasAssignment {
    name: String,
    due_at: Option<String>,
    html_url: String,
}

/// represents a Canvas assignment
#[derive(Clone)]
pub struct CanvasAssignment {
    name: String,
    due_date: Option<DateTime<Local>>,
    url: Url,
}

impl CanvasAssignment {
    /// creates a new CanvasAssignment from a PartialCanvasAssignment
    fn new(partial: PartialCanvasAssignment) -> anyhow::Result<CanvasAssignment> {
        Ok(CanvasAssignment {
            name: partial.name,
            due_date: match partial.due_at {
                Some(due_date) => Some(due_date.parse::<DateTime<Local>>()?),
                None => None,
            },
            url: Url::parse(partial.html_url.as_str())?,
        })
    }

    /// returns the assignment's name
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// returns the assignment's due date
    pub fn due_date(&self) -> Option<DateTime<Local>> {
        self.due_date
    }

    /// returns the assignment's url
    pub fn url(&self) -> Url {
        self.url.clone()
    }
}

/// a deserializable Canvas To Do without proper representation of data
/// todo: doesn't account for quizzes
#[derive(serde::Deserialize, Clone)]
struct PartialCanvasTodo {
    assignment: Option<PartialCanvasAssignment>,
}

/// todo: doesn't account for quizzes (also leave a proper doc comment)
pub struct CanvasTodo {
    assignment: Option<CanvasAssignment>,
}

impl CanvasTodo {
    /// creates a new CanvasTodo from a PartialCanvasTodo
    fn new(partial: PartialCanvasTodo) -> anyhow::Result<CanvasTodo> {
        Ok(CanvasTodo {
            assignment: match partial.assignment {
                Some(assignment) => Some(CanvasAssignment::new(assignment)?),
                None => None,
            },
        })
    }

    /// returns the to do's assignment
    pub fn assignment(&self) -> Option<CanvasAssignment> {
        self.assignment.clone()
    }
}

/// a struct that represents Canvas API information
pub struct CanvasAPI {
    token: String,
}

impl CanvasAPI {
    /// creates a new CanvasAPI from a token
    pub fn new(token: String) -> CanvasAPI {
        CanvasAPI { token }
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

    /// gets all todos
    pub async fn get_todos(&self) -> Result<Vec<CanvasTodo>, reqwest::Error> {
        // get the url to request to
        let url = format!(
            "https://ames.instructure.com/api/v1/users/self/todo?access_token={}",
            self.token
        );

        // get a response from the api and return it
        let response = reqwest::get(&url)
            .await?
            .json::<Vec<PartialCanvasTodo>>()
            .await?;
        let parsed: Vec<_> = response
            .iter()
            .map(|partial| CanvasTodo::new(partial.clone()).expect("Failed to parse Canvas TODO"))
            .collect();
        Ok(parsed)
    }
}
