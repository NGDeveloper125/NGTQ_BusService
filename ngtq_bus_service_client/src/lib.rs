use std::{io::{Read, Write}, os::unix::net::UnixStream};
use models::{BusRequest, CategoryTask, IdTask, Task, TaskIdentifier};

mod models;

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
            return Err(String::from("Failed to send task - BusServiceClient is not initilised!"))
        }

        match id_task.is_valid() {
            Ok(_) => {

                let serialise_task = serde_json::to_string(&BusRequest::PushTask(Task::Id(id_task))).unwrap();
                match send_request_to_bus(serialise_task, &self.bus_address) {
                    Ok(response) => {
                        match response.parse::<usize>() {
                            Ok(queue_size) => Ok(queue_size),
                            Err(error) => Err(format!("Failed to parse queue size to usize: {}", error.to_string()))
                        }
                    },
                    Err(error) => Err(error)
                }
            },
            Err(error) => return Err(error)
        }
    }

    pub fn send_category_task_to_bus(&self, category_task: CategoryTask) -> Result<String, String> {
        if !self.is_initialise {
            return Err(String::from("Failed to send task - BusServiceClient is not initilised"));
        }

        match category_task.is_valid() {
            Ok(_) => {
                let serialised_task = serde_json::to_string(&BusRequest::PushTask(Task::Category(category_task))).unwrap();
                match send_request_to_bus(serialised_task, &self.bus_address) {
                    Ok(response) => Ok(response),
                    Err(error) => Err(error)
                }
            },
            Err(error) => Err(error)
        }
    }

    pub fn pull_id_task_from_bus(&self, id: String) -> Result<String, String> {
        if !self.is_initialise {
            return Err(String::from("Failed to pull task - BusServiceClient is not initilised"));
        }
        
        if id.is_empty() {
            return Err(String::from("Failed to pull task - Id can not be empty"));
        }

        let serialised_task = serde_json::to_string(&BusRequest::PullTask(TaskIdentifier::Id(id))).unwrap();
        match send_request_to_bus(serialised_task, &self.bus_address) {
            Ok(response) => Ok(response),
            Err(error) => Err(error)
        }
    }
}

fn send_request_to_bus(serialise_request: String, bus_address: &str) -> Result<String, String> {
    let buffer = serialise_request.as_bytes();
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
