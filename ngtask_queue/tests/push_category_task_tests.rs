use ngtask_queue::{CategoryTask, TaskQueue};


#[test]
fn valid_new_message_test_push_new_category_task_to_queue() {
    let task_queue_arc = TaskQueue::initialise();
    let task = CategoryTask {
        category: String::from("test"),
        payload: String::from("Do this and that")
    };

    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            let (number_of_queue, queue_len) = task_queue.push_category_task(task);
            assert_eq!(number_of_queue, 1);
            assert_eq!(queue_len, 1);
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
            let (number_of_queue, queue_len) = task_queue.push_category_task(task);
            assert_eq!(number_of_queue, 1);
            assert_eq!(queue_len, 1);
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
            let (number_of_queue, queue_len) = task_queue.push_category_task(task);
            assert_eq!(number_of_queue, 1);
            assert_eq!(queue_len, 2);
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
            let (number_of_queue, queue_len) = task_queue.push_category_task(task);
            assert_eq!(number_of_queue, 0);
            assert_eq!(queue_len, 0);
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
            let (number_of_queue, queue_len) = task_queue.push_category_task(task);
            assert_eq!(number_of_queue, 0);
            assert_eq!(queue_len, 0);
        },
        Err(error) => {
            println!("Failed to open queue: {:?}", error);
            assert!(false)
        }
    };
}