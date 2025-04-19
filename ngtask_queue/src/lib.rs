use std::{collections::HashMap, sync::{atomic::{AtomicUsize, Ordering}, Arc, Mutex}};
use ngtq::{ NGTQError, NGTQ };

static IDCOUNTER: AtomicUsize = AtomicUsize::new(0);
pub struct TaskQueue {
    pub is_initialised: bool,
    pub id_queue: HashMap<String, String>,
    pub category_queues: HashMap<String, Vec<String>>,
}

impl TaskQueue {
    fn initialised_check(&self, issue_desc: &str) -> Result<(), NGTQError> {
        if !self.is_initialised {
            return Err(
                NGTQError::generate_error(
                    ngtq::NGTQErrorType::Initialisation(String::from("The object was not initialised")), 
                    String::from(issue_desc)
                ))
        }
        Ok(())
    }
}


impl NGTQ for TaskQueue {
    fn initialise() -> Arc<Mutex<TaskQueue>> {
        let is_initialised = true;
        let id_queue: HashMap<String, String> = HashMap::new();
        let category_queues: HashMap<String, Vec<String>> = HashMap::new();

        Arc::new(Mutex::new(TaskQueue { is_initialised, id_queue, category_queues }))
    }

    fn get_id_queue_len(&self) -> Result<usize, NGTQError> {
        match self.initialised_check("Failed to get queue length") {
            Ok(_) => {
                Ok(self.id_queue.len())
            },
            Err(error) => Err(error)
        }
    }

    fn get_category_queue_len(&self, category: &str) -> Result<usize, NGTQError> {
        match self.initialised_check("Failed to get queue length") {
            Ok(_) => {
                match self.category_queues.get(category) {
                    Some(queue) => Ok(queue.len()),
                    None => Err(
                        NGTQError::generate_error(
                            ngtq::NGTQErrorType::CategoryQueue(String::from("No queue found for this category")), 
                            String::from("Failed to get queue length")))
                }
            },
            Err(error) => Err(error)
        }
    }



    fn push_id_task_to_queue(&mut self, payload: String) -> Result<String, NGTQError> {
        match self.initialised_check("Failed to push new task") {
            Ok(_) => {
                let id = generate_id();
                
                if payload == String::new() {
                    Err(
                        NGTQError::generate_error(
                            ngtq::NGTQErrorType::IdQueue(String::from("The task payload is empty")), 
                            String::from("Failed to push new task")
                        )
                    )
                } else {
                    match self.id_queue.insert(id.to_string(), payload) {
                        Some(_) => Err(
                            NGTQError::generate_error(
                                ngtq::NGTQErrorType::IdQueue(String::from("A task with this id exist in the queue")),
                                String::from("Fatal Error")
                            )
                        ), 
                        None => Ok(id)
                    }
                }
            },
            Err(error) => Err(error)
        }
    }
    
    fn push_category_task_to_queue(&mut self, category: String, payload: String) -> Result<(), NGTQError> {
        match self.initialised_check("Failed to push new task") {
            Ok(_) => {
                if category == String::new() || payload == String::new() {
                    return Err(
                        NGTQError::generate_error(
                            ngtq::NGTQErrorType::CategoryQueue(String::from("The task category or payload is empty")),
                            String::from("Failed to push new task")
                        )
                    )
                } 
                match self.category_queues.get_mut(&category) {
                    Some(queue) => {
                        push_category_task_to_existing_queue(queue, payload);
                        Ok(())
                    },
                    None => {
                        let new_queue = vec![payload];
                        self.category_queues.insert(category, new_queue);
                        Ok(())
                    }
                }
            },
            Err(error) => Err(error)
        }
    }
    
    fn pull_id_task_from_queue(&mut self, id: &str) -> Result<String, NGTQError> {
        match self.initialised_check("Failed to pull task") {
            Ok(_) => {
                match self.id_queue.remove(id) {
                    Some(payload) => Ok(payload),
                    None => Err(
                        NGTQError::generate_error(
                            ngtq::NGTQErrorType::IdQueue(String::from("Task with this id was not found in queue")),
                            String::from("Failed to pull task from queue")
                        )
                    )
                }
            },
            Err(error) => Err(error)
        }
    }
    
    fn pull_category_task_from_queue(&mut self, category: &str) -> Result<String, NGTQError> {
        match self.initialised_check("Failed to pull task") {
            Ok(_) => {
                match self.category_queues.remove(category) {
                    Some(mut queue) => {
                        if queue.len() > 1 {
                            let payload = queue.remove(0);
                            self.category_queues.insert(category.to_string(), queue);
                            Ok(payload)
                        } else {
                            Ok(queue.remove(0))
                        }
                    },
                    None => Err(
                        NGTQError::generate_error(
                            ngtq::NGTQErrorType::CategoryQueue(String::from("No tasks for this topic were found")),
                            String::from("Failed to pull task from queue")
                        )
                    )
                }
            },
            Err(error) => Err(error)
        }
    }
}

fn push_category_task_to_existing_queue(queue: &mut Vec<String>, payload: String) -> usize {
    queue.push(payload);
    queue.len()
}

fn generate_id() -> String {
    IDCOUNTER.fetch_add(1, Ordering::SeqCst).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_initialised() {
        let task_queue_arc = TaskQueue::initialise();
        match task_queue_arc.lock() {
            Ok(task_queue) => assert_eq!(task_queue.is_initialised, true),
            Err(_) => assert!(false)
        };
    }

    #[test]
    fn id_queue_len() {
        let task_queue_arc = TaskQueue::initialise();

        match task_queue_arc.lock() {
            Ok(mut task_queue) => {
                assert_eq!(task_queue.get_id_queue_len().unwrap(), 0);
                match task_queue.push_id_task_to_queue(String::from("Do Somthing")) {
                    Ok(_) => assert_eq!(task_queue.get_id_queue_len().unwrap(), 1),
                    Err(error) => {
                        println!("Test Failed - {}", error);
                        assert!(false)
                    }
                }
            },
            Err(error) => {
                println!("Test Failed -failed to open queue: {}", error);
                assert!(false)
            }
        };
    }


    #[test]
    fn category_queue_len() {
        let task_queue_arc = TaskQueue::initialise();

        match task_queue_arc.lock() {
            Ok(mut task_queue) => {
                match task_queue.push_category_task_to_queue(String::from("test"), String::from("Do Somthing")) {
                    Ok(_) => {
                        match task_queue.get_category_queue_len("test") {
                            Ok(queue_size) => assert_eq!(queue_size, 1),
                            Err(error) => {
                                println!("Test Failed - {}", error);
                                assert!(false)
                            }
                        }
                    },
                    Err(error) => {
                        println!("Test Failed - failed to push task to queue: {}", error);
                        assert!(false)
                    }
                };
            }
            Err(error) => {
                println!("Test Failed - failed to open queue: {}", error);
                assert!(false)
            }
        };
    }
}
