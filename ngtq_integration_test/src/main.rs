use std::{process::{Child, Command}, thread, time::Duration};

fn main() {
    println!("Run 'cargo test' to execute");
}

pub fn start_bus() -> Child {
    println!("Starting bus service...");
    let child = Command::new("../ngtq_bus_service/target/debug/ngtq_bus_service")
                                  .spawn()
                                  .expect("Failed to start bus service");
    thread::sleep(Duration::from_secs(2));

    child
}

#[cfg(test)]
mod tests {
    use super::*;
    use ngtq_bus_service_client::{self, BusServiceClient, CategoryTask, IdTask};

    #[test]
    fn ngtq_bus_server_client_lib_test_id_task() {

        let mut bus_service = start_bus();

        let bus_service_client = BusServiceClient::initialise("/tmp/resu_ipc_socket".to_string());

        let id_task = IdTask {
            id: "test1".to_string(),
            payload: "Do somthing".to_string()
        };

        match bus_service_client.send_id_task_to_bus(id_task) {
            Ok(_) => (),
            Err(error) => {
                println!("Failed test: {}", error);
                assert!(false)
            }
        }

        match bus_service_client.pull_id_task_from_bus(("test1").to_string()) {
            Ok(_) => (),
            Err(error) => {
                println!("Failed test - {}", error);
                assert!(false)
            }
        }

        bus_service.kill().expect("Failed to kill application process");
        assert!(true)

    }

    #[test]
    fn ngtq_bus_server_client_lib_test_category_task() {

        let mut bus_service = start_bus();

        let bus_service_client = BusServiceClient::initialise("/tmp/resu_ipc_socket".to_string());

        let category_task = CategoryTask {
            category: "test1".to_string(),
            payload: "Do somthing".to_string()
        };

        match bus_service_client.send_category_task_to_bus(category_task) {
            Ok(_) => (),
            Err(error) => {
                println!("Failed test: {}", error);
                assert!(false)
            }
        }

        match bus_service_client.pull_category_task_from_bus(("test1").to_string()) {
            Ok(_) => (),
            Err(error) => {
                println!("Failed test - {}", error);
                assert!(false)
            }
        }

        bus_service.kill().expect("Failed to kill application process");
        assert!(true)

    }
}
