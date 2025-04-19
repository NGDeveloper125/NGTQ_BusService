use ngtask_queue::TaskQueue;
use ngtq::NGTQ;


#[test]
fn valid_new_message_test_push_new_category_task_to_queue() {
    let task_queue_arc = TaskQueue::initialise();

    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            match task_queue.push_category_task_to_queue(String::from("test"), String::from("Do this and that")) {
                Ok(()) => {
                    match task_queue.get_category_queue_len("test") {
                        Ok(queue_size) => {
                            assert_eq!(task_queue.category_queues.len(), 1);
                            assert_eq!(queue_size, 1);
                        },
                        Err(error) => {
                            println!("Test Failed - failed to get queue length: {}", error);
                            assert!(false)
                        }        
                    }
                },
                Err(error) => {
                    println!("Test Failed - {}", error);
                    assert!(false)
                }
            }
        },
        Err(error) => {
            println!("Test Failed - failed to open queue: {:?}", error);
            assert!(false)
        }
    };
}

#[test]
fn valid_new_message_test_push_existing_category_task_to_queue() {
    let task_queue_arc = TaskQueue::initialise();

    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            match task_queue.push_category_task_to_queue(String::from("test"), String::from("Do this and that")) {
                Ok(()) => {
                    match task_queue.get_category_queue_len("test") {
                        Ok(queue_size) => {
                            assert_eq!(task_queue.category_queues.len(), 1);
                            assert_eq!(queue_size, 1);
                        },
                        Err(error) => {
                            println!("Test Failed - failed to get queue length: {}", error);
                            assert!(false)
                        }        
                    }      
                },
                Err(error) => {
                    println!("Test Failed - {}", error);
                    assert!(false)
                }
            }
        },
        Err(error) => {
            println!("Test Failed - failed to open queue: {:?}", error);
            assert!(false)
        }
    };

    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            match task_queue.push_category_task_to_queue(String::from("test"), String::from("Do this and that")) {
                Ok(()) => {
                    match task_queue.get_category_queue_len("test") {
                        Ok(queue_size) => {
                            assert_eq!(task_queue.category_queues.len(), 1);
                            assert_eq!(queue_size, 2);
                        },
                        Err(error) => {
                            println!("Test Failed - failed to get queue length: {}", error);
                            assert!(false)
                        }        
                    }    
                },
                Err(error) => {
                    println!("Test Failed - {}", error);
                    assert!(false)
                }
            }
        },
        Err(error) => {
            println!("Test Failed - failed to open queue: {:?}", error);
            assert!(false)
        }
    };
}

#[test]
fn invalid_category_new_message_test_push_category_task_to_queue() {
    let task_queue_arc = TaskQueue::initialise();

    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            match task_queue.push_category_task_to_queue(String::new(), String::from("Do this and that")) {
                Ok(_) => {
                    println!("Test Failed - expected an error");
                    assert!(false)
                },
                Err(_) => assert!(true)
            }
        },
        Err(error) => {
            println!("Test Failed - failed to open queue: {:?}", error);
            assert!(false)
        }
    };
}

#[test]
fn invalid_payload_new_message_test_push_category_task_to_queue() {
    let task_queue_arc = TaskQueue::initialise();

    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            match task_queue.push_category_task_to_queue(String::from("test"), String::new()) {
                Ok(_) => {
                    println!("Test Failed - expected an error");
                    assert!(false)
                },
                Err(_) => assert!(true)
            }
        },
        Err(error) => {
            println!("Test Failed - failed to open queue: {:?}", error);
            assert!(false)
        }
    };
}