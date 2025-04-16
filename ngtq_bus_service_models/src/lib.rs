use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum BusRequest {
    PushTask(Task),
    PullTask(TaskIdentifier)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskIdentifier {
    Id(String),
    Category(String)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Task {
    Id(String),
    Category(String, String)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusResponse {
    pub successful: bool,
    pub error: Option<String>,
    pub payload: Option<String>
}
