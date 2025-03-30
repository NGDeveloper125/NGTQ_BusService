use ngtask_queue::{IdTask, TaskQueue};

#[test]
fn id_not_in_queue_test_pull_id_task_from_queue() {
    let task_queue_arc = TaskQueue::initialise();
    
    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            let queue_response = task_queue.pull_id_task(String::from("test"));
            assert_eq!(queue_response, None)
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
            let queue_response = task_queue.pull_id_task(String::new());
            assert_eq!(queue_response, None)
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
    let task = IdTask {
        id: String::from("test"),
        payload: payload.clone()
    };

    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            task_queue.push_id_task(task);
            match task_queue.pull_id_task(String::from("test")) {
                Some(payloud) => {
                    assert_eq!(payloud, payload);
                    assert_eq!(task_queue.get_id_queue_len().unwrap(), 0)
                },
                None => assert!(false)
            }
        },
        Err(error) => {
            println!("Failed to open queue {:?}", error);
            assert!(false)
        }
    };
}