use std::{io::{Read, Write}, os::unix::net::UnixStream};
pub use models::{BusRequest, BusResponse, CategoryTask, IdTask, Task, TaskIdentifier};
mod models;

pub struct BusServiceClient {
    is_initialise: bool,
    bus_address: String
}

impl BusServiceClient {
    pub fn initialise(bus_address: String) -> BusServiceClient {
        BusServiceClient { is_initialise: true, bus_address: bus_address }
    }

    pub fn send_id_task_to_bus(&self, id_task: IdTask) -> Result<Option<String>, String> {
        if !self.is_initialise {
            return Err(String::from("Failed to send task - BusServiceClient is not initilised!"))
        }

        match id_task.is_valid() {
            Ok(_) => {

                let serialise_task = serde_json::to_string(&BusRequest::PushTask(Task::Id(id_task))).unwrap();
                match send_request_to_bus(serialise_task, &self.bus_address) {
                    Ok(serialised_response) => handle_bus_response(serialised_response),
                    Err(error) => Err(format!("Failed to parse queue size to usize: {}", error.to_string()))
                }
            },
            Err(error) => return Err(error)
        }
    }

    pub fn send_category_task_to_bus(&self, category_task: CategoryTask) -> Result<Option<String>, String> {
        if !self.is_initialise {
            return Err(String::from("Failed to send task - BusServiceClient is not initilised"));
        }

        match category_task.is_valid() {
            Ok(_) => {
                let serialised_task = serde_json::to_string(&BusRequest::PushTask(Task::Category(category_task))).unwrap();
                match send_request_to_bus(serialised_task, &self.bus_address) {
                    Ok(serialised_response) => handle_bus_response(serialised_response),
                    Err(error) => Err(error)
                }
            },
            Err(error) => Err(error)
        }
    }

    pub fn pull_id_task_from_bus(&self, id: String) -> Result<Option<String>, String> {
        if !self.is_initialise {
            return Err(String::from("Failed to pull task - BusServiceClient is not initilised"));
        }
        
        if id.is_empty() {
            return Err(String::from("Failed to pull task - Id can not be empty"));
        }

        let serialised_request = serde_json::to_string(&BusRequest::PullTask(TaskIdentifier::Id(id))).unwrap();
        match send_request_to_bus(serialised_request, &self.bus_address) {
            Ok(serialised_response) => handle_bus_response(serialised_response),
            Err(error) => Err(error)
        }
    }

    pub fn pull_category_task_from_bus(&self, category: String) -> Result<Option<String>, String> {
        if !self.is_initialise {
            return Err(String::from("Failed to pull task - BusServiceClient is not initilised"));
        }

        if category.is_empty() {
            return Err(String::from("Faield to pull task - Category can not be empty"));
        }

        let serialised_request = serde_json::to_string(&BusRequest::PullTask(TaskIdentifier::Category(category))).unwrap();
        match send_request_to_bus(serialised_request, &self.bus_address) {
            Ok(serialised_response) => handle_bus_response(serialised_response),
            Err(error) => Err(error)
        }
    }
}

fn send_request_to_bus(serialised_request: String, bus_address: &str) -> Result<String, String> {
    let buffer = serialised_request.as_bytes();
    match UnixStream::connect(bus_address) {
        Ok(mut stream) => {
            
            match stream.write_all(buffer) {
                Ok(_) => {
                    let mut response: String = String::new();
                    match stream.read_to_string(&mut response) {
                        Ok(_) => Ok(response),
                        Err(error) => Err(format!("Failed to receive response from bus: {}", error))
                    }
                },
                Err(error) => Err(format!("Failed to send to bus: {}", error))
            }
        },
        Err(error) => Err(format!("Failed to connect: {}", error))
    }
}

fn handle_bus_response(serialised_response: String) -> Result<Option<String>, String> {
    println!("serialised_response: {}", serialised_response);
    let response: BusResponse = match serde_json::from_str(&serialised_response) {
        Ok(response) => response,
        Err(error) => {
            BusResponse {
                successful: false,
                error: Some(format!("Failed to deserialise response from bus {}", error.to_string())),
                payload: None
            }
        }
    };
    if response.successful {
        return Ok(response.payload)
    }
    Err(response.error.unwrap())
}