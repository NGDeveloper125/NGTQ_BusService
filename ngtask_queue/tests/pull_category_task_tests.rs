use ngtask_queue::{CategoryTask, TaskQueue};
use ngtq::NGTQ;

#[test]
fn no_queue_for_the_category_exist_test_pull_category_task() {
    let task_queue_arc = TaskQueue::initialise();
    
    match task_queue_arc.lock() {
        Ok(mut queue) => {
            let pull_task_result = queue.pull_category_task_from_queue(String::from("test"));
            assert_eq!(pull_task_result, Err(String::from("Failed to pull task from queue - no tasks for this topic were found")))
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
            let i = task_queue.push_category_task_to_queue(task1);
            assert_eq!(i, Ok(()));
            let e = task_queue.push_category_task_to_queue(task2);
            assert_eq!(e, Ok(()));

            match task_queue.get_category_queue_len("test") {
                Ok(queue_size) => assert_eq!(queue_size, 2),
                Err(error) => {
                    println!("Failed to get queue length: {}", error);
                    assert!(false)
                }
            }

            match task_queue.pull_category_task_from_queue(String::from("test")) {
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
    let task = CategoryTask {
        category: String::from("test"),
        payload: task_payload.to_string()
    };

    match task_queue_arc.lock() {
        Ok(mut queue) => {
            let i = queue.push_category_task_to_queue(task);
            assert_eq!(i, Ok(()));

            match queue.pull_category_task_from_queue(String::from("test")) {
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
