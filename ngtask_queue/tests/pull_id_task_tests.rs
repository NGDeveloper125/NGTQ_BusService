use ngtask_queue::TaskQueue;
use ngtq::NGTQ;

#[test]
fn id_not_in_queue_test_pull_id_task_from_queue() {
    let task_queue_arc = TaskQueue::initialise();
    
    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            match task_queue.pull_id_task_from_queue("test"){
                Ok(_) => {
                 println!("Test Failed - expected to fail because no queue for this category exist");
                 assert!(false)
                },
                Err(_) => assert!(true)
             }
        },
        Err(error) => {
            println!("Test Failed - failed to open queue {:?}", error);
            assert!(false)
        }
    };
}

#[test]
fn id_not_valid_test_pull_id_task_from_queue() {
    let task_queue_arc = TaskQueue::initialise();
    
    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            match task_queue.pull_id_task_from_queue("") {
                Ok(_) => {
                 println!("Test Failed - expected to fail because no queue for this category exist");
                 assert!(false)
                },
                Err(_) => assert!(true)
             }
        },
        Err(error) => {
            println!("Test Failed - failed to open queue {:?}", error);
            assert!(false)
        }
    };
}

#[test]
fn id_valid_and_exist_test_pull_id_task_from_queue() {
    let task_queue_arc = TaskQueue::initialise();
    let payload = String::from("Do something"); 

    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            match task_queue.push_id_task_to_queue(payload.to_string()) {
                Ok(id) => {
                    match task_queue.pull_id_task_from_queue(&id) {
                        Ok(payloud) => {
                            assert_eq!(payloud, payload);
                            assert_eq!(task_queue.get_id_queue_len().unwrap(), 0)
                        },
                        Err(_) => assert!(false)
                    }
                },
                Err(error) => {
                    println!("Test Failed - failed to push task to queue: {}", error);
                    assert!(false)
                }
            }
        },
        Err(error) => {
            println!("Test Failed - failed to open queue {:?}", error);
            assert!(false)
        }
    };
}