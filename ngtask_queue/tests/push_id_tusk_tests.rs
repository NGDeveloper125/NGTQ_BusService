use ngtask_queue::{Id, IdTask, TaskQueue};
use ngtq::{NGId, NGTQ};


#[test]
fn valid_new_message_test_push_id_task_to_queue() {
    let task_queue_arc = TaskQueue::initialise();
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
        payload: String::from("Do Somthing")
    };

    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            match task_queue.push_id_task_to_queue::<IdTask, Id>(task) {
               Ok(_) => {
                    match task_queue.get_id_queue_len() {
                        Ok(queue_size) => assert_eq!(queue_size, 1),
                        Err(error) => {
                            println!("Failed to get queue size: {}", error);
                            assert!(false)
                        }
                    }
                },
                Err(error) => {
                    println!("Failed to push task to queue: {}", error);
                    assert!(false)
                } 
            }
        },
        Err(error) => {
            println!("Failed to open queue: {:?}", error);
            assert!(false)
        }
    };
}

#[test]
fn valid_existing_id_message_test_push_id_task_to_queue() {
    let task_queue_arc = TaskQueue::initialise();
    let id = match Id::set_with_validation(String::from("1234567890")) {
        Ok(valid_id) => Some(valid_id),
        Err(error) => {
            println!("Test Failed: failed to create task id: {}", error);
            assert!(false);
            None
        }
    };
    let task1 = IdTask {
        id: id,
        payload: String::from("Do Somthing")
    };

    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            match task_queue.push_id_task_to_queue::<IdTask, Id>(task1) {
                Ok(_) => {
                    match task_queue.get_id_queue_len() {
                        Ok(queue_size) => assert_eq!(queue_size, 1),
                        Err(error) => {
                            println!("Failed to get queue size: {}", error);
                            assert!(false)
                        }
                    }
                },
                Err(error) => {
                    println!("Failed to push task to queue: {}", error);
                    assert!(false)
                }
            }
        },
        Err(error) => {
            println!("Failed to open queue: {:?}", error);
            assert!(false)
        }
    };

    let id = match Id::set_with_validation(String::from("1234567890")) {
        Ok(valid_id) => Some(valid_id),
        Err(error) => {
            println!("Test Failed: failed to create task id: {}", error);
            assert!(false);
            None
        }
    };
    let task2 = IdTask {
        id: id,
        payload: String::from("Do Somthing")
    };
    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            match task_queue.push_id_task_to_queue::<IdTask, Id>(task2) {
                Ok(_) => {
                 println!("Expected to fail because no queue for this category exist");
                 assert!(false)
                },
                Err(_) => assert!(true)
             }
            assert_eq!(task_queue.get_id_queue_len().unwrap(), 1)
        },
        Err(error) => {
            println!("Failed to open queue: {:?}", error);
            assert!(false)
        }
    };
}

#[test]
fn invalid_new_message_test_push_id_task_to_queue() {
    let task_queue_arc = TaskQueue::initialise();
    let id = match Id::set_with_validation(String::from("1234")) {
        Ok(valid_id) => Some(valid_id),
        Err(error) => {
            None
        }
    };
    let task = IdTask {
        id: id,
        payload: String::from("Do Somthing")
    };
    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            match task_queue.push_id_task_to_queue::<IdTask, Id>(task) {
                Ok(_) => {
                 println!("Expected to fail because no id is invalid");
                 assert!(false)
                },
                Err(_) => assert!(true)
             }
            assert_eq!(task_queue.get_id_queue_len().unwrap(), 0)
        },
        Err(error) => {
            println!("Failed to open queue: {:?}", error);
            assert!(false)
        }
    };
}