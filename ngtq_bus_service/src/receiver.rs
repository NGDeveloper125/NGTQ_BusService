
use ngtask_queue::{CategoryTask, IdTask, TaskQueue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum IncomingRequest {
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
enum Task {
    Id(IdTask),
    Category(CategoryTask),
    Error(String)
}

pub struct Receiver {

}

impl Receiver  {
    
}