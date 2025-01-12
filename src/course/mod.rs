use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseWrapper {
    pub body: Course,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Course {
    pub id: String,
    pub enrolled: bool,
    pub chapters: Vec<Chapter>,
    pub current_topic: Option<String>,
    pub category: Option<String>,
    pub image: Option<String>,
    pub title: Option<String>,
    pub enrollable: bool,
    pub status: Option<String>,
    pub level: Option<String>,
    pub duration: Option<String>,
    pub description: String,
    pub description_paragraphs: Vec<String>,
    pub description_summary: String,
    pub learning_items: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chapter {
    pub id: String,
    pub title: String,
    pub topics: Vec<Topic>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub topic_type: Option<String>,
    pub next: Option<String>,
}
