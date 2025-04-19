use std::{io::{Read, Write}, os::unix::net::UnixStream};

use ngtq_bus_service_models::{BusRequest, BusResponse, Task, TaskIdentifier};

pub struct BusServiceClient {
    is_initialise: bool,
    bus_address: String
}

impl BusServiceClient {
    pub fn initialise(bus_address: String) -> BusServiceClient {
        BusServiceClient { is_initialise: true, bus_address }
    }

    pub fn send_task_to_bus(&self, task_payload: String) -> Result<Option<String>, String> {
        if !self.is_initialise {
            return Err(String::from("Failed to send task - BusServiceClient is not initilised!"))
        }


        if task_payload.is_empty() {
            return Err(String::from("Failed to pull task - payload can not be empty"));
        }

        match serde_json::to_string::<BusRequest>(&BusRequest::PushTask(Task::Id(task_payload))) {
            Ok(serialise_task) => {
                match send_request_to_bus(serialise_task, &self.bus_address) {
                    Ok(serialised_response) => handle_bus_response(serialised_response),
                    Err(error) => Err(format!("Failed to parse queue size to usize: {}", error))
                }
            },
            Err(error) => Err(format!("Failed to serialise task to send to bus: {}", error))
        }
    }
    

    pub fn send_task_to_bus_with_category(&self, task_category: String, task_payload: String) -> Result<Option<String>, String> {
        if !self.is_initialise {
            return Err(String::from("Failed to send task - BusServiceClient is not initilised"));
        }

        if task_category.is_empty() {
            return Err(String::from("Failed to pull task - category can not be empty"));
        }


        if task_payload.is_empty() {
            return Err(String::from("Failed to pull task - payload can not be empty"));
        }


        match serde_json::to_string::<BusRequest>(&BusRequest::PushTask(Task::Category(task_category, task_payload))) {
            Ok(serialised_task) => {
                match send_request_to_bus(serialised_task, &self.bus_address) {
                    Ok(serialised_response) => handle_bus_response(serialised_response),
                    Err(error) => Err(error)
                }
            },
            Err(error) => Err(format!("Failed to serialise bus request: {}", error))
        }
    }
    

    pub fn pull_task_from_bus(&self, id: String) -> Result<Option<String>, String> {
        if !self.is_initialise {
            return Err(String::from("Failed to pull task - BusServiceClient is not initilised"));
        }
        
        if id.is_empty() {
            return Err(String::from("Failed to pull task - Id can not be empty"));
        }

        match serde_json::to_string::<BusRequest>(&BusRequest::PullTask(TaskIdentifier::Id(id))) {
            Ok(serialised_request) => {
                match send_request_to_bus(serialised_request, &self.bus_address) {
                    Ok(serialised_response) => handle_bus_response(serialised_response),
                    Err(error) => Err(error)
                }
            },
            Err(error) => Err(format!("Failed to serialised bus request: {}", error))
        }
    }

    pub fn pull_task_from_bus_by_category(&self, category: String) -> Result<Option<String>, String> {
        if !self.is_initialise {
            return Err(String::from("Failed to pull task - BusServiceClient is not initilised"));
        }

        if category.is_empty() {
            return Err(String::from("Faield to pull task - Category can not be empty"));
        }

        match serde_json::to_string::<BusRequest>(&BusRequest::PullTask(TaskIdentifier::Category(category))) {
            Ok(serialised_request) => {
                match send_request_to_bus(serialised_request, &self.bus_address) {
                    Ok(serialised_response) => handle_bus_response(serialised_response),
                    Err(error) => Err(error)
                }
            },
            Err(error) => Err(format!("Failed to serialise bus request: {}", error))
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
                error: Some(format!("Failed to deserialise response from bus {}", error)),
                payload: None
            }
        }
    };
    if response.successful {
        return Ok(response.payload)
    }
    Err(response.error.unwrap().to_string())
}