use std::{process::{Child, Command}, thread, time::Duration};

fn main() {
    println!("Run 'cargo test' to execute");
}

pub fn start_bus() -> Child {
    println!("Starting bus service...");
    let child = Command::new("../target/debug/ngtq_bus_service")
                                  .spawn()
                                  .expect("Failed to start bus service");
    thread::sleep(Duration::from_secs(2));

    child
}

#[cfg(test)]
mod tests {
    use super::*;
    use ngtq_bus_service_client::BusServiceClient;
    use serial_test::serial;

    #[test]
    #[serial]
    fn ngtq_bus_server_client_lib_test_id_task() {

        let mut bus_service = start_bus();

        let bus_service_client = BusServiceClient::initialise("/tmp/resu_ipc_socket".to_string());

        match bus_service_client.send_task_to_bus(String::from("Do somthing")) {
            Ok(id_option) => { 
                    match id_option {
                    Some(task_id) => {
                        match bus_service_client.pull_task_from_bus(task_id) {
                            Ok(_) => (),
                            Err(error) => {
                                println!("Failed test - {}", error);
                                assert!(false)
                            }
                        }
                    },
                    None => {
                        println!("Test failed - failed to get back id after pushing task");
                        assert!(false);
                    } 
                }
            },
            Err(error) => {
                println!("Failed test: {}", error);
                assert!(false);
            }
        };

        bus_service.kill().expect("Failed to kill application process");
        assert!(true)

    }

    #[test]
    #[serial]
    fn ngtq_bus_server_client_lib_test_category_task() {

        let mut bus_service = start_bus();

        let bus_service_client = BusServiceClient::initialise("/tmp/resu_ipc_socket".to_string());

        match bus_service_client.send_task_to_bus_with_category(String::from("category test"), String::from("Do Somthing")) {
            Ok(_) => {
                match bus_service_client.pull_task_from_bus_by_category(String::from("category test")) {
                    Ok(_) => (),
                    Err(error) => {
                        println!("Failed test - {}", error);
                        assert!(false)
                    }
                }
            },
            Err(error) => {
                println!("Failed test: {}", error);
                assert!(false)
            }
        }

        bus_service.kill().expect("Failed to kill application process");
        assert!(true)

    }
}
