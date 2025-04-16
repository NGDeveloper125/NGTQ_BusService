use ngtask_queue::TaskQueue;
use ngtq::NGTQ;


#[test]
fn valid_new_message_test_push_id_task_to_queue() {
    let task_queue_arc = TaskQueue::initialise();

    match task_queue_arc.lock() {
        Ok(mut task_queue) => {
            match task_queue.push_id_task_to_queue(String::from("Do Somthing")) {
               Ok(_) => {
                    match task_queue.get_id_queue_len() {
                        Ok(queue_size) => assert_eq!(queue_size, 1),
                        Err(error) => {
                            println!("Failed to get queue size: {}", error);
                            assert!(false)
                        }
                    }
                },
                Err(error) => {
                    println!("Failed to push task to queue: {}", error);
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