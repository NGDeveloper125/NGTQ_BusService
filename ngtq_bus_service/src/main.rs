use std::{fs::File, io::BufReader};

use clap::Parser;
use ngtask_queue::TaskQueue;
use config::BusConfig;

mod receiver;
mod config;

// config file path
const CONFILG_FILE_PATH: &str = "ngtq_bus_service_config.json"; 

#[derive(Parser)]
#[clap(author, version, about = "Task-Queue Bus Service Application")]
struct Args {
    #[clap(short, long)]
    socket_path: Option<String>,
}

fn load_config() -> Result<BusConfig, Box<dyn std::error::Error>> {
    let file = File::open(CONFILG_FILE_PATH)?;
    let reader = BufReader::new(file);
    let config = serde_json::from_reader(reader)?;
    Ok(config)
}

fn main() -> std::io::Result<()> {
    println!("Starting Bus Service...");

    println!("Loading configuration...");
    let args = Args::parse();
    let socket_path = match args.socket_path {
        Some(path) => path,
        None => {
            match load_config() {
                Ok(config) => config.socket_path,
                Err(_) => "/tmp/resu_ipc_socket".to_string()
            }
        }
    };

    let receiver_instance = receiver::Receiver {

    };

    match receiver_instance.start_receiver::<TaskQueue>(&socket_path, &true) {
        Ok(_) => (),
        Err(error) => println!("Recevier failed: {}", error)
    }

    Ok(())
}
