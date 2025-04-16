use ngtask_queue::TaskQueue;
use ngtq::NGTQ;

#[test]
fn no_queue_for_the_category_exist_test_pull_category_task() {
    let task_queue_arc = TaskQueue::initialise();
    
    match task_queue_arc.lock() {
        Ok(mut queue) => {
            match queue.pull_category_task_from_queue("test") {
               Ok(_) => {
                println!("Expected to fail because no queue for this category exist");
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
fn queue_for_the_category_exist_test_pull_category_task() {
    let task_queue_arc = TaskQueue::initialise();
    let task_payload = String::from("Do This");

    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            match task_queue.push_category_task_to_queue(String::from("test"), task_payload.to_string()) {
                Ok(_) => assert!(true),
                Err(error) => {
                    println!("Expected to return ok: {}", error);
                    assert!(false)
                }
            }
            match task_queue.push_category_task_to_queue(String::from("test"), task_payload.to_string()) {
                Ok(_) => assert!(true),
                Err(error) => {
                    println!("Expected to return ok: {}", error);
                    assert!(false)    
                }
            }
            match task_queue.get_category_queue_len("test") {
                Ok(queue_size) => assert_eq!(queue_size, 2),
                Err(error) => {
                    println!("Failed to get queue length: {}", error);
                    assert!(false)
                }
            }

            match task_queue.pull_category_task_from_queue("test") {
                Ok(payload) => {
                    assert_eq!(payload, task_payload);
                },
                Err(error)=> {
                    println!("{}", error);
                    assert!(false)
                }
            }

            assert_eq!(task_queue.get_category_queue_len("test").unwrap(), 1)
        },
        Err(error) => {
            println!("Failed to open queue: {:?}", error);
            assert!(false)
        }
    };
}

#[test]
fn queue_for_the_category_exist_with_last_task_test_pull_category_task() {
    let task_queue_arc = TaskQueue::initialise();
    let task_payload = String::from("Do This");

    match task_queue_arc.lock() {
        Ok(mut queue) => {
            match queue.push_category_task_to_queue(String::from("test"), task_payload.to_string()){
                Ok(_) => assert!(true),
                Err(error) => {
                    println!("Expected to return ok: {}", error);
                    assert!(false)    
                }
            }
            match queue.pull_category_task_from_queue("test") {
                Ok(payload) => {
                    assert_eq!(payload, task_payload);
                    match queue.get_category_queue_len("test") {
                        Ok(_) => {
                            println!("Queue exists while should be empty");
                            assert!(false)
                        },
                        Err(_) => assert!(true)
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
