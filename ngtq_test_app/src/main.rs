use std::{io::{Read, Write}, os::unix::net::UnixStream, thread};

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

fn main() -> std::io::Result<()> {
    let mut handlers = Vec::new();
    let mut switch = true;
    for thread_num in 1..200 {
        if switch {
            let handler = thread::spawn(move || {
                let socket_path = "/tmp/resu_ipc_socket";

                let id_task = IdTask {
                    id: thread_num.to_string(),
                    payload: "Do somthing".to_string()
                };

                let serialise_task = serde_json::to_string(&IncomingRequest::PushTask(Task::Id(id_task))).unwrap();
                let buffer = serialise_task.as_bytes();
                match UnixStream::connect(socket_path) {
                    Ok(mut stream) => {
                        
                        match stream.write_all(buffer) {
                            Ok(_) => {
                                let mut response: String = String::new();
                                match stream.read_to_string(&mut response) {
                                    Ok(_) => println!("sender {} have finished!", thread_num),
                                    Err(error) => println!("Failed to receive response from bus: {}", error)
                                }
                            },
                            Err(error) => println!("Failed to send to bus: {}", error)
                        }
                    },
                    Err(error) => println!("Failed to connect: {}", error)
                }
            });
            handlers.push(handler);
            switch = false;
        } else {
            let handler = thread::spawn(move || {
                let socket_path = "/tmp/resu_ipc_socket";

                let task_identifier = TaskIdentifier::Id((thread_num - 1).to_string());

                let serialise_task = serde_json::to_string(&IncomingRequest::PullTask(task_identifier)).unwrap();
                let buffer = serialise_task.as_bytes();
                match UnixStream::connect(socket_path) {
                    Ok(mut stream) => {
                        
                        match stream.write_all(buffer) {
                            Ok(_) => {
                                let mut response: String = String::new();
                                match stream.read_to_string(&mut response) {
                                    Ok(_) => println!("receiver {} have finished! received: {}", thread_num -1, response),
                                    Err(error) => println!("Failed to receive response from bus: {}", error)
                                }
                            },
                            Err(error) => println!("Failed to send to bus: {}", error)
                        }
                    },
                    Err(error) => println!("Failed to connect: {}", error)
                }
            });
            handlers.push(handler);
            switch = true;
        }
    }

    let mut handler_num = 0;
    for handler in handlers {
        match handler.join() {
            Ok(_) => (),
            Err(error) => println!("Handler failed: {:?}", error)
        }
    }

    Ok(())
}
