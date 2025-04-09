use std::{io::{Read, Write}, os::unix::net::UnixStream};
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
    bus_address: String
}

impl BusServiceClient {
    pub fn initialise(bus_address: String) -> BusServiceClient {
        BusServiceClient { is_initialise: true, bus_address: bus_address }
    }

    pub fn send_id_task_to_bus(&self, id_task: IdTask) -> Result<usize, String> {
        if !self.is_initialise {
            return Err(String::from("Failed to send task - BusServiceClient is not intilised!"))
        }

        let serialise_task = serde_json::to_string(&BusRequest::PushTask(Task::Id(id_task))).unwrap();
        let buffer = serialise_task.as_bytes();
        match UnixStream::connect(&self.bus_address) {
            Ok(mut stream) => {
                
                match stream.write_all(buffer) {
                    Ok(_) => {
                        let mut response: String = String::new();
                        match stream.read_to_string(&mut response) {
                            Ok(_) => {
                                match response.parse::<usize>() {
                                    Ok(queue_size) => Ok(queue_size),
                                    Err(error) => Err(format!("Failed to parse queue size to usize: {}", error.to_string()))
                                }
                            },
                            Err(error) => Err(format!("Failed to receive response from bus: {}", error))
                        }
                    },
                    Err(error) => Err(format!("Failed to send to bus: {}", error))
                }
            },
            Err(error) => Err(format!("Failed to connect: {}", error))
        }
    }

}
