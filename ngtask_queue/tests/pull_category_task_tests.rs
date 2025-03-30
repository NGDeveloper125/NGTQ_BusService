use ngtask_queue::{CategoryTask, TaskQueue};

#[test]
fn no_queue_for_the_category_exist_test_pull_category_task() {
    let task_queue_arc = TaskQueue::initialise();
    
    match task_queue_arc.lock() {
        Ok(mut queue) => {
            let pull_task_result = queue.pull_category_task(String::from("test"));
            assert_eq!(pull_task_result, None)
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
    let task1 = CategoryTask {
        category: String::from("test"),
        payload: task_payload.to_string()
    };

    let task2 = CategoryTask {
        category: String::from("test"),
        payload: task_payload.to_string()
    };

    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            let i = task_queue.push_category_task(task1);
            assert_eq!(i, (1,1));
            let e = task_queue.push_category_task(task2);
            assert_eq!(e, (1,2));

            match task_queue.pull_category_task(String::from("test")) {
                Some(payload) => {
                    assert_eq!(payload, task_payload);
                },
                None => {
                    println!("Failed to find task in queue");
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
    let task = CategoryTask {
        category: String::from("test"),
        payload: task_payload.to_string()
    };

    match task_queue_arc.lock() {
        Ok(mut queue) => {
            let i = queue.push_category_task(task);
            assert_eq!(i, (1,1));

            match queue.pull_category_task(String::from("test")) {
                Some(payload) => {
                    assert_eq!(payload, task_payload);
                    match queue.get_category_queue("test") {
                        Some(_) => {
                            println!("Queue exists while should be empty");
                            assert!(false)
                        },
                        None => assert!(true)
                    }
                },
                None => {
                    println!("Failed to find task in queue");
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
