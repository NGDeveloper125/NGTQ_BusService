use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum BusRequest {
    PushTask(Task),
    PullTask(TaskIdentifier),
    Error(String)
}

#[derive(Debug, Serialize, Deserialize)]
enum TaskIdentifier {
    Id(String),
    Category(String),
    Error(String)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdTask {
    id: String,
    payload: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryTask {
    id: String,
    payload: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Task {
    Id(IdTask),
    Category(CategoryTask),
    Error(String)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusResponse {
    successful: bool, 
    error: String, 
    payload: String
}

pub struct BusServiceClient {
    is_initialise: bool,
    busa_address: String
}

impl BusServiceClient {
    
}
