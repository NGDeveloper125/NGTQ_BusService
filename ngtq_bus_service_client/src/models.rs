

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum BusRequest {
    PushTask(Task),
    PullTask(TaskIdentifier),
    Error(String)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskIdentifier {
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
    category: String,
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

impl CategoryTask {
    pub fn is_valid(&self) -> Result<(), String> {
        if self.category == String::new() {
            return Err(String::from("Task is not valid - category can not be empty"))
        }

        if self.payload == String::new() {
            return Err(String::from("Task is not valid - Payload can not be empty"))
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::{CategoryTask, IdTask};


    #[test]
    fn is_valid_id_tasktest_id_is_invalid_return_error() {
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

    #[test]
    fn is_valid_id_task_test_payload_is_invalid_return_error() {
        let task = IdTask {
            id: String::from("test"),
            payload: String::new()
        };

        match task.is_valid() {
            Ok(_) => {
                println!("Expected to return error");
                assert!(false)
            },
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn is_valid_category_task_test_category_is_invalid_return_error() {
        let task = CategoryTask {
            category: String::new(),
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

    #[test]
    fn is_valid_category_task_test_payload_is_invalid_return_error() {
        let task = CategoryTask {
            category: String::from("test"),
            payload: String::new()
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
