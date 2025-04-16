
use std::{io::{Read, Write}, os::unix::net::{UnixListener, UnixStream}, sync::{mpsc::{self, Sender}, Arc, Mutex}, thread};
use ngtq::NGTQ;
use ngtq_bus_service_models::{BusRequest, BusResponse, Task, TaskIdentifier};

pub struct Receiver {

}

impl Receiver  {
    pub fn start_receiver<T: NGTQ>(&self, socket_path: &str, keep_running: &bool) -> Result<(), String> {
        let wrapped_task_queue = T::initialise();
        let (tx, rx): (mpsc::Sender<UnixStream>, mpsc::Receiver<UnixStream>) = mpsc::channel();

        start_receiving(socket_path.to_string(), tx);

        while *keep_running {
            match rx.recv() {
                Ok(mut stream) => {
                    let mut buffer: [u8; 1024] = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(bytes) => {
                            let incoming_request = String::from_utf8_lossy(&buffer[..bytes]);
                            let response = handle_incoming_request(incoming_request.to_string(), &wrapped_task_queue);
                            match serde_json::to_string(&response) {
                                Ok(serialised_response ) => {
                                    match stream.write_all(serialised_response.as_bytes()) {
                                        Ok(_) => (), // log?
                                        Err(error) => return Err(error.to_string())
                                    }
                                },
                                Err(error) => return Err(error.to_string())
                            }
                        },
                        Err(error) => return Err(error.to_string())
                    }
                },
                Err(error) => return Err(error.to_string())    
            }
        }
        Ok(())
    }
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

fn handle_incoming_request<T: NGTQ>(incoming_request: String, wrapped_task_queue: &Arc<Mutex<T>>) -> BusResponse {
    let deserialized_request: BusRequest = match serde_json::from_str(&incoming_request) {
        Ok(incoming_request) => incoming_request,
        Err(error) => return BusResponse 
            { 
                successful: false, 
                error: Some(format!("Deserialisation Failed: {}", error)), 
                payload: None 
            }
    };

    match deserialized_request {
        BusRequest::PushTask(task) => handle_push_request(task, wrapped_task_queue),
        BusRequest::PullTask(task_identifier) => handle_pull_request(task_identifier, wrapped_task_queue),
    }
}

fn handle_push_request<T: NGTQ>(task: Task, wrapped_task_queue: &Arc<Mutex<T>>) -> BusResponse {
    match task {
        Task::Id(payload) => handle_id_task_push_request(payload, wrapped_task_queue),
        Task::Category(category, payload) => handle_category_task_push_request(category, payload, wrapped_task_queue)
    }
}

fn handle_pull_request<T: NGTQ>(task_identifier: TaskIdentifier, wrapped_task_queue: &Arc<Mutex<T>>) -> BusResponse {
    match task_identifier {
        TaskIdentifier::Id(task_id) => handle_id_task_pull_request(task_id, wrapped_task_queue),
        TaskIdentifier::Category(task_category) => handle_category_task_pull_request(task_category, wrapped_task_queue)
    }
}

fn handle_id_task_push_request<T: NGTQ>(payload: String, wrapped_task_queue: &Arc<Mutex<T>>) -> BusResponse {
    match wrapped_task_queue.lock() {
        Ok(mut task_queue) => {
            match task_queue.push_id_task_to_queue(payload) {
                Ok(id) => BusResponse { successful: true, error: None, payload: Some(id) },
                Err(error) => BusResponse { successful: false, error: Some(format!("{}", error)), payload: None }
            }
        },
        Err(error) => BusResponse 
            { 
                successful: false, 
                error: Some(format!("Could not open wrapped task queue: {}", error)), 
                payload: None 
            } 
    }
}

fn handle_category_task_push_request<T: NGTQ>(category: String, payload: String, wrapped_task_queue: &Arc<Mutex<T>>) -> BusResponse {
    match wrapped_task_queue.lock() {
        Ok(mut task_queue) => {
            match task_queue.push_category_task_to_queue(category, payload) {
                Ok(_) => BusResponse { successful: true, error: None, payload: None },
                Err(error) => BusResponse { successful: false, error: Some(format!("{}", error)), payload: None }
            }
        },
        Err(error) => return BusResponse 
        { 
            successful: false, 
            error: Some(format!("Could not open wrapped task queue {}", error)),
            payload: None 
        }
    }
}

fn handle_id_task_pull_request<T: NGTQ>(task_id: String, wrapped_task_queue: &Arc<Mutex<T>>) -> BusResponse {
    match wrapped_task_queue.lock() {
        Ok(mut task_queue) => {
            match task_queue.pull_id_task_from_queue(&task_id) {
                Ok(payload) => BusResponse { successful: true, error: None, payload: Some(payload) },
                Err(error) => BusResponse { successful: false, error: Some(format!("{}", error)), payload: None }
            }
        },
        Err(error) => return BusResponse 
        { 
            successful: false, 
            error: Some(format!("Could not open wrapped task queue: {}", error)),
            payload: None 
        }
    }
}

fn handle_category_task_pull_request<T: NGTQ>(task_category: String, wrapped_task_queue: &Arc<Mutex<T>>) -> BusResponse {
    match wrapped_task_queue.lock() {
        Ok(mut task_queue) => {
            match task_queue.pull_category_task_from_queue(&task_category) {
                Ok(payload) => BusResponse { successful: true, error: None, payload: Some(payload) },
                Err(error) => BusResponse { successful: false, error: Some(format!("{}", error)), payload: None }
            }
        },
        Err(error) => return BusResponse 
        { 
            successful: false, 
            error: Some(format!("Could not open wrapped task queue: {}", error)),
            payload: None 
        }
    }
}