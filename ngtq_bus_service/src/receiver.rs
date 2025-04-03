
use std::{os::unix::net::{UnixListener, UnixStream}, sync::mpsc::Sender, thread};

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