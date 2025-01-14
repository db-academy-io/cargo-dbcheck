use crate::error::DbCheckError;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseResponseWrapper {
    pub body: CourseSyllabus,
}

impl TryFrom<serde_json::Value> for CourseResponseWrapper {
    type Error = DbCheckError;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value).map_err(|e| DbCheckError::FormatError(e.to_string()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseSyllabus {
    pub id: String,
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
    pub chapters: Vec<Chapter>,
    pub url: String,
}

impl TryFrom<serde_json::Value> for CourseSyllabus {
    type Error = DbCheckError;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value).map_err(|e| DbCheckError::FormatError(e.to_string()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chapter {
    pub id: String,
    pub title: String,
    pub topics: Vec<Topic>,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub topic_type: Option<String>,
    pub next_url: Option<String>,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseStatus {
    pub id: String,

    pub enrolled: bool,

    #[serde(rename = "currentTopic")]
    pub current_topic: Option<String>,

    #[serde(rename = "currentChapter")]
    pub current_chapter: Option<String>,

    #[serde(rename = "completedTopics", default = "Vec::new")]
    pub completed: Vec<String>,
}

impl TryFrom<serde_json::Value> for CourseStatus {
    type Error = DbCheckError;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value).map_err(|e| DbCheckError::FormatError(e.to_string()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseStatusResponseWrapper {
    pub body: CourseStatus,
}

impl TryFrom<serde_json::Value> for CourseStatusResponseWrapper {
    type Error = DbCheckError;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value).map_err(|e| DbCheckError::FormatError(e.to_string()))
    }
}
