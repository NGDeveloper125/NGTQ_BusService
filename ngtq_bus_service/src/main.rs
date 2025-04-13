use ngtask_queue::{CategoryTask, IdTask, TaskQueue};

mod receiver;

fn main() -> std::io::Result<()> {
    println!("Starting Bus Service...");
    let receiver_instance = receiver::Receiver {

    };

    match receiver_instance.start_receiver::<TaskQueue, IdTask, CategoryTask>( "/tmp/resu_ipc_socket", &true) {
        Ok(_) => println!("Is this reachable?"),
        Err(error) => println!("Recevier failed: {}", error)
    }

    Ok(())
}
