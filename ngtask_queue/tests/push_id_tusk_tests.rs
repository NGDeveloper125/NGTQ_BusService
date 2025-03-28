use ngtask_queue::{IdTask, TaskQueue};


#[test]
fn valid_new_message_test_push_id_task_to_queue() {
    let task_queue_arc = TaskQueue::initialise();
    let id_task = IdTask {
        id: String::from("1"),
        payload: String::from("Do this and that")
    };

    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            let i = task_queue.push_id_task(id_task);
            assert_eq!(i, 1);
            assert_eq!(task_queue.id_queue.len(), 1)
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
            let i = task_queue.push_id_task(id_task1);
            assert_eq!(i, 1);
            assert_eq!(task_queue.id_queue.len(), 1)
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
            let i = task_queue.push_id_task(id_task2);
            assert_eq!(i, 0);
            assert_eq!(task_queue.id_queue.len(), 1)
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
            let i = task_queue.push_id_task(id_task);
            assert_eq!(i, 0);
            assert_eq!(task_queue.id_queue.len(), 0)
        },
        Err(error) => {
            println!("Failed to open queue: {:?}", error);
            assert!(false)
        }
    };
}