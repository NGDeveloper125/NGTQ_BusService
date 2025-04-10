

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum BusRequest {
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

impl IdTask {
    pub fn is_valid(&self) -> Result<(), String> {
        if self.id == String::new() {
            return Err(String::from("Task is not valid - Id can not be empty"))
        }

        if self.payload == String::new() {
            return Err(String::from("Task is not valid - Payload can not be empty"))
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::IdTask;


    #[test]
    fn is_valid_test_id_is_invalid_return_error() {
        let task = IdTask {
            id: String::new(),
            payload: String::from("test")
        };

        match task.is_valid() {
            Ok(_) => {
                println!("Expected to return error");
                assert!(false)
            },
            Err(_) => assert!(true)
        }
    }
}
