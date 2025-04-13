use ngtask_queue::{IdTask, TaskQueue};
use ngtq::NGTQ;

#[test]
fn id_not_in_queue_test_pull_id_task_from_queue() {
    let task_queue_arc = TaskQueue::initialise();
    
    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            let queue_response = task_queue.pull_id_task_from_queue(String::from("test"));
            assert_eq!(queue_response, Err(String::from("Failed to pull task from queue - task with this id was not found in queue")))
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
            let queue_response = task_queue.pull_id_task_from_queue(String::new());
            assert_eq!(queue_response, Err(String::from("Failed to pull task from queue - task with this id was not found in queue")))
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
            match task_queue.push_id_task_to_queue(task) {
                Ok(_) => {
                    match task_queue.pull_id_task_from_queue(String::from("test")) {
                        Ok(payloud) => {
                            assert_eq!(payloud, payload);
                            assert_eq!(task_queue.get_id_queue_len().unwrap(), 0)
                        },
                        Err(_) => assert!(false)
                    }
                },
                Err(error) => {
                    println!("Failed to push task to queue: {}", error);
                    assert!(false)
                }
            }
        },
        Err(error) => {
            println!("Failed to open queue {:?}", error);
            assert!(false)
        }
    };
}