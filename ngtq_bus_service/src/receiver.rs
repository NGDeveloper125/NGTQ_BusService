
use std::{os::unix::net::{UnixListener, UnixStream}, string, sync::{mpsc::Sender, Arc, Mutex}, thread};

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

#[derive(Debug, Serialize, Deserialize)]
struct BusResponse {
    pub successful: bool,
    pub error: String,
    pub payload: String
}

pub struct Receiver {

}

impl Receiver  {
    
}

fn start_receiving(socket_path: String, tx: Sender<UnixStream>) {
    thread::spawn(move || {
        std::fs::remove_file(&socket_path).ok();

        match UnixListener::bind(&socket_path) {
            Ok(listener) => {
                println!("Bus is receiving on: {}", &socket_path);

                for incoming_stream in listener.incoming() {
                    match incoming_stream {
                        Ok(stream) => {
                            match tx.send(stream) {
                                Ok(_) => {},
                                Err(error) => println!("Failed to send incoming stream: {}", error)
                            }
                        },
                        Err(error) => println!("Failed to get incoming stream: {}", error)
                    }
                }
            },
            Err(error) => println!("Failed to bind receiver: {}", error)
        }
    });
}

fn handle_incoming_request(incoming_request: String, wrapped_task_queue: &Arc<Mutex<TaskQueue>>) -> BusResponse {
    let deserialized_request: IncomingRequest = match serde_json::from_str(&incoming_request) {
        Ok(incoming_request) => incoming_request,
        Err(error) => return BusResponse { successful: false, error: error.to_string(), payload: String::new() }
    };

    match deserialized_request {
        IncomingRequest::PushTask(task) => handle_push_request(task, wrapped_task_queue),
        IncomingRequest::PullTask(task_identifier) => handle_pull_request(task_identifier, wrapped_task_queue),
        IncomingRequest::Error(error) => BusResponse { successful: false, error: error, payload: String::new() }
    }
}

fn handle_push_request(task: Task, wrapped_task_queue: &Arc<Mutex<TaskQueue>>) -> BusResponse {
    match task {
        Task::Id(id_task) => handle_id_task_push_request(id_task, wrapped_task_queue),
        Task::Category(category_task) => handle_category_task_push_request(category_task, wrapped_task_queue),
        Task::Error(error) => BusResponse { successful: false, error: error, payload: String::new() }
    }
}

fn handle_pull_request(task_identifier: TaskIdentifier, wrapped_task_queue: &Arc<Mutex<TaskQueue>>) -> BusResponse {
    match task_identifier {
        TaskIdentifier::Id(task_id) => handle_id_task_pull_request(task_id, wrapped_task_queue),
        TaskIdentifier::Category(task_category) => handle_category_task_pull_request(task_category, wrapped_task_queue),
        TaskIdentifier::Error(error) => BusResponse { successful: false, error: error, payload: String::new() }
    }
}

fn handle_id_task_push_request(task: IdTask, wrapped_task_queue: &Arc<Mutex<TaskQueue>>) -> BusResponse {
    match wrapped_task_queue.lock() {
        Ok(mut task_queue) => {
            match task_queue.push_id_task(task) {
                Ok(_) => BusResponse { successful: true, error: String::new(), payload: String::new() },
                Err(error) => BusResponse { successful: false, error: error, payload: String::new() }
            }
        },
        Err(error) => {
            println!("Failed to push id task: {}", error);
            return BusResponse { successful: false, error: error.to_string(), payload: String::new() } 
        }
    }
}

fn handle_category_task_push_request(task: CategoryTask, wrapped_task_queue: &Arc<Mutex<TaskQueue>>) -> BusResponse {
    match wrapped_task_queue.lock() {
        Ok(mut task_queue) => {
            match task_queue.push_category_task(task) {
                Ok(_) => BusResponse { successful: true, error: String::new(), payload: String::new() },
                Err(error) => BusResponse { successful: false, error: error.to_string(), payload: String::new() }
            }
        },
        Err(error) => {
            println!("Failed to push category task: {}", error);
            return BusResponse { successful: false, error: error.to_string(), payload: String::new() };
        }
    }
}

fn handle_id_task_pull_request(task_id: String, wrapped_task_queue: &Arc<Mutex<TaskQueue>>) -> BusResponse {
    match wrapped_task_queue.lock() {
        Ok(mut task_queue) => {
            match task_queue.pull_id_task(task_id) {
                Ok(payload) => BusResponse { successful: true, error: String::new(), payload: payload },
                Err(error) => BusResponse { successful: false, error: error, payload: String::new() }
            }
        },
        Err(error) => BusResponse { successful: false, error: error.to_string(), payload: String::new() }
    }
}

fn handle_category_task_pull_request(task_category: String, wrapped_task_queue: &Arc<Mutex<TaskQueue>>) -> BusResponse {
    match wrapped_task_queue.lock() {
        Ok(mut task_queue) => {
            match task_queue.pull_category_task(task_category) {
                Ok(payload) => BusResponse { successful: true, error: String::new(), payload: payload },
                Err(error) => BusResponse { successful: false, error: error, payload: String::new() }
            }
        },
        Err(error) => BusResponse { successful: false, error: error.to_string(), payload: String::new() }
    }
}