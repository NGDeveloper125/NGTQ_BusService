use ngtask_queue::{Id, IdTask, TaskQueue};
use ngtq::{NGId, NGTQ};

#[test]
fn id_not_in_queue_test_pull_id_task_from_queue() {
    let task_queue_arc = TaskQueue::initialise();
    
    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            match task_queue.pull_id_task_from_queue(String::from("test")){
                Ok(_) => {
                 println!("Expected to fail because no queue for this category exist");
                 assert!(false)
                },
                Err(_) => assert!(true)
             }
        },
        Err(error) => {
            println!("Failed to open queue {:?}", error);
            assert!(false)
        }
    };
}

#[test]
fn id_not_valid_test_pull_id_task_from_queue() {
    let task_queue_arc = TaskQueue::initialise();
    
    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            match task_queue.pull_id_task_from_queue(String::new()) {
                Ok(_) => {
                 println!("Expected to fail because no queue for this category exist");
                 assert!(false)
                },
                Err(_) => assert!(true)
             }
        },
        Err(error) => {
            println!("Failed to open queue {:?}", error);
            assert!(false)
        }
    };
}

#[test]
fn id_valid_and_exist_test_pull_id_task_from_queue() {
    let task_queue_arc = TaskQueue::initialise();
    let payload = String::from("Do something"); 
    let id = match Id::set_with_validation(String::from("1234567890")) {
        Ok(valid_id) => Some(valid_id),
        Err(error) => {
            println!("Test Failed: failed to create task id: {}", error);
            assert!(false);
            None
        }
    };
    let task = IdTask {
        id: id,
        payload: payload.clone()
    };

    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            match task_queue.push_id_task_to_queue::<IdTask, Id>(task) {
                Ok(_) => {
                    match task_queue.pull_id_task_from_queue(String::from("1234567890")) {
                        Ok(payloud) => {
                            assert_eq!(payloud, payload);
                            assert_eq!(task_queue.get_id_queue_len().unwrap(), 0)
                        },
                        Err(_) => assert!(false)
                    }
                },
                Err(error) => {
                    println!("Failed to push task to queue: {}", error);
                    assert!(false)
                }
            }
        },
        Err(error) => {
            println!("Failed to open queue {:?}", error);
            assert!(false)
        }
    };
}