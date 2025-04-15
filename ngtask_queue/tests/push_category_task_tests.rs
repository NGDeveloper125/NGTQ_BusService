use ngtask_queue::{CategoryTask, TaskQueue};
use ngtq::NGTQ;


#[test]
fn valid_new_message_test_push_new_category_task_to_queue() {
    let task_queue_arc = TaskQueue::initialise();
    let task = CategoryTask {
        category: String::from("test"),
        payload: String::from("Do this and that")
    };

    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            match task_queue.push_category_task_to_queue(task) {
                Ok(()) => {
                    match task_queue.get_category_queue_len("test") {
                        Ok(queue_size) => {
                            assert_eq!(task_queue.category_queues.len(), 1);
                            assert_eq!(queue_size, 1);
                        },
                        Err(error) => {
                            println!("Failed to get queue length: {}", error);
                            assert!(false)
                        }        
                    }
                },
                Err(error) => {
                    println!("{}", error);
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
fn valid_new_message_test_push_existing_category_task_to_queue() {
    let task_queue_arc = TaskQueue::initialise();
    let task = CategoryTask {
        category: String::from("test"),
        payload: String::from("Do this and that")
    };

    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            match task_queue.push_category_task_to_queue(task) {
                Ok(()) => {
                    match task_queue.get_category_queue_len("test") {
                        Ok(queue_size) => {
                            assert_eq!(task_queue.category_queues.len(), 1);
                            assert_eq!(queue_size, 1);
                        },
                        Err(error) => {
                            println!("Failed to get queue length: {}", error);
                            assert!(false)
                        }        
                    }      
                },
                Err(error) => {
                    println!("{}", error);
                    assert!(false)
                }
            }
        },
        Err(error) => {
            println!("Failed to open queue: {:?}", error);
            assert!(false)
        }
    };

    let task = CategoryTask {
        category: String::from("test"),
        payload: String::from("Do this and that")
    };

    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            match task_queue.push_category_task_to_queue(task) {
                Ok(()) => {
                    match task_queue.get_category_queue_len("test") {
                        Ok(queue_size) => {
                            assert_eq!(task_queue.category_queues.len(), 1);
                            assert_eq!(queue_size, 2);
                        },
                        Err(error) => {
                            println!("Failed to get queue length: {}", error);
                            assert!(false)
                        }        
                    }    
                },
                Err(error) => {
                    println!("{}", error);
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
fn invalid_category_new_message_test_push_category_task_to_queue() {
    let task_queue_arc = TaskQueue::initialise();
    let task = CategoryTask {
        category: String::new(),
        payload: String::from("Do this and that")
    };

    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            match task_queue.push_category_task_to_queue(task) {
                Ok(_) => {
                    println!("Expected an error");
                    assert!(false)
                },
                Err(_) => assert!(true)
            }
        },
        Err(error) => {
            println!("Failed to open queue: {:?}", error);
            assert!(false)
        }
    };
}

#[test]
fn invalid_payload_new_message_test_push_category_task_to_queue() {
    let task_queue_arc = TaskQueue::initialise();
    let task = CategoryTask {
        category: String::from("test"),
        payload: String::new()
    };

    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            match task_queue.push_category_task_to_queue(task) {
                Ok(_) => {
                    println!("Expected an error");
                    assert!(false)
                },
                Err(_) => assert!(true)
            }
        },
        Err(error) => {
            println!("Failed to open queue: {:?}", error);
            assert!(false)
        }
    };
}