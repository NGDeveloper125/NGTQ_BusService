use ngtask_queue::{IdTask, TaskQueue};
use ngtq::NGTQ;


#[test]
fn valid_new_message_test_push_id_task_to_queue() {
    let task_queue_arc = TaskQueue::initialise();
    let id_task = IdTask {
        id: String::from("1"),
        payload: String::from("Do this and that")
    };

    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            match task_queue.push_id_task_to_queue(id_task) {
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
    let id_task1: IdTask = IdTask {
        id: String::from("1"),
        payload: String::from("Do this and that")
    };

    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            match task_queue.push_id_task_to_queue(id_task1) {
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

    let id_task2: IdTask = IdTask {
        id: String::from("1"),
        payload: String::from("Do this and that")
    };
    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            match task_queue.push_id_task_to_queue(id_task2) {
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
    let id_task = IdTask {
        id: String::new(),
        payload: String::from("Do this and that")
    };

    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            match task_queue.push_id_task_to_queue(id_task) {
                Ok(_) => {
                 println!("Expected to fail because no queue for this category exist");
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