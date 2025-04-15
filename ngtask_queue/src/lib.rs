use std::{collections::HashMap, sync::{Arc, Mutex}};
use ngtq::{ NGCategoryTask, NGId, NGIdTask, NGTQError, NGTQ };
pub use task::{ Id, IdTask, CategoryTask};

mod task;
pub struct TaskQueue {
    pub is_initialised: bool,
    pub id_queue: HashMap<String, String>,
    pub category_queues: HashMap<String, Vec<String>>,
}

impl NGTQ for TaskQueue {
    fn initialise() -> Arc<Mutex<TaskQueue>> {
        let is_initialised = true;
        let id_queue: HashMap<String, String> = HashMap::new();
        let category_queues: HashMap<String, Vec<String>> = HashMap::new();

        Arc::new(Mutex::new(TaskQueue { is_initialised, id_queue, category_queues }))
    }

    fn get_id_queue_len(&self) -> Result<usize, NGTQError> {
        if !self.is_initialised {
            return Err(
                NGTQError::generate_error(
                    ngtq::NGTQErrorType::Initialisation(String::from("The TaskQueue was not initialised")), 
                    String::from("Failed to get queue length")
                )
            )
        }
        Ok(self.id_queue.len())
    }

    fn get_category_queue_len(&self, category: &str) -> Result<usize, NGTQError> {
        if !self.is_initialised {
            return Err(
                NGTQError::generate_error(
                    ngtq::NGTQErrorType::Initialisation(String::from("The TaskQueue was not initialised")), 
                    String::from("Failed to get queue length")
                )
            )
        }
        match self.category_queues.get(category) {
            Some(queue) => return  Ok(queue.len()),
            None => return Err(
                NGTQError::generate_error(
                    ngtq::NGTQErrorType::CategoryQueue(String::from("No queue found for this category")), 
                    String::from("Failed to get queue length")))
        }
    }



    fn push_id_task_to_queue<T: NGIdTask, TaskId: NGId>(&mut self, task: T) -> Result<(), NGTQError> {
        if !self.is_initialised {
            return Err(
                NGTQError::generate_error(
                    ngtq::NGTQErrorType::Initialisation(String::from("The TaskQueue was not initialised")), 
                    String::from("Failed to push new task")
                )
            )
        }

        let task_id = match task.get_id::<TaskId>() {
            Some(id) => id,
            None => return Err(
                NGTQError::generate_error(
                    ngtq::NGTQErrorType::IdQueue(String::from("The task id object was empty")), 
                    String::from("Task need to have a valid id")
                )
            )
        };
        
        if task.get_payload() == String::new() {
            return Err(
                NGTQError::generate_error(
                    ngtq::NGTQErrorType::IdQueue(String::from("The task id or payload is empty")), 
                    String::from("Failed to push new task")
                )
            )
        } else if self.id_queue.contains_key(&task_id) {
            return Err(
                NGTQError::generate_error(
                    ngtq::NGTQErrorType::IdQueue(String::from("A task with this id already exist in the queue")),
                    String::from("Failed to push new task")
                )
            )    
        } else {
            return match self.id_queue.insert(task_id, task.get_payload()) {
                Some(_) => return Err(
                    NGTQError::generate_error(
                        ngtq::NGTQErrorType::IdQueue(String::from("A task with this id exist in the queuea")),
                        String::from("Fatal Error")
                    )
                ), 
                None => Ok(())
            }
        }
    }
    
    fn push_category_task_to_queue<T: NGCategoryTask>(&mut self, task: T) -> Result<(), NGTQError> {
        if !self.is_initialised {
            return Err(
                NGTQError::generate_error(
                    ngtq::NGTQErrorType::Initialisation(String::from("The TaskQueue was not initialised")),
                    String::from("Failed to push new task")
                )
            )
        }
        if task.get_category() == String::new() || task.get_payload() == String::new() {
            return Err(
                NGTQError::generate_error(
                    ngtq::NGTQErrorType::CategoryQueue(String::from("The task topic or payload is empty")),
                    String::from("Failed to push new task")
                )
            )
        } 
        match self.category_queues.get_mut(task.get_category()) {
            Some(queue) => {
                push_category_task_to_existing_queue(queue, task);
                return Ok(())
            },
            None => {
                let mut new_queue = Vec::new();
                new_queue.push(task.get_payload());
                self.category_queues.insert(task.get_category().to_string(), new_queue);
                return Ok(());
            }
        }
    }
    
    fn pull_id_task_from_queue(&mut self, id: String) -> Result<String, NGTQError> {
        if !self.is_initialised {
            return Err(
                NGTQError::generate_error(
                    ngtq::NGTQErrorType::Initialisation(String::from("The TaskQueue was not initialised")),
                    String::from("Failed to pull task")
                )
            )
        }
        match self.id_queue.remove(&id) {
            Some(payload) => Ok(payload),
            None => Err(
                NGTQError::generate_error(
                    ngtq::NGTQErrorType::IdQueue(String::from("Task with this id was not found in queue")),
                    String::from("Failed to pull task from queue")
                )
            )
        }
    }
    
    fn pull_category_task_from_queue(&mut self, category: String) -> Result<String, NGTQError> {
        if !self.is_initialised {
            return Err(
                NGTQError::generate_error(
                    ngtq::NGTQErrorType::Initialisation(String::from("The TaskQueue was not initialised")),
                    String::from("Failed to pull task")
                )
            )
        }
        match self.category_queues.remove(&category) {
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
    }
}

fn push_category_task_to_existing_queue<T: NGCategoryTask>(queue: &mut Vec<String>, task: T) -> usize {
    queue.push(task.get_payload());
    queue.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_initialised() {
        let task_queue_arc = TaskQueue::initialise();
        let result = match task_queue_arc.lock() {
            Ok(task_queue) => task_queue.is_initialised,
            Err(_) => false
        };
        assert_eq!(result, true)
    }

    #[test]
    fn id_queue_len() {
        let task_queue_arc = TaskQueue::initialise();
        let id = match Id::set_with_validation(String::from("1234567890")) {
            Ok(valid_id) => Some(valid_id),
            Err(error) => {
                println!("Test Failed: failed to create task id: {}", error);
                assert!(false);
                None
            }
        };
        let task = IdTask {
            id: id,
            payload: String::from("Do Somthing")
        };

        match task_queue_arc.lock() {
            Ok(mut task_queue) => {
                assert_eq!(task_queue.get_id_queue_len().unwrap(), 0);
                match task_queue.push_id_task_to_queue::<IdTask, Id>(task) {
                    Ok(_) => assert_eq!(task_queue.get_id_queue_len().unwrap(), 1),
                    Err(error) => {
                        println!("{}", error);
                        assert!(false)
                    }
                }
            },
            Err(error) => {
                println!("Failed to open queue: {}", error);
                assert!(false)
            }
        };
    }


    #[test]
    fn category_queue_len() {
        let task_queue_arc = TaskQueue::initialise();
        let category = String::from("test");
        let task = CategoryTask {
            category: category.to_string(),
            payload: String::from("Do Somthing")
        };

        match task_queue_arc.lock() {
            Ok(mut task_queue) => {
                task_queue.push_category_task_to_queue(task).expect("Failed to push task to queue");
                match task_queue.get_category_queue_len("test") {
                    Ok(queue_size) => assert_eq!(queue_size, 1),
                    Err(error) => {
                        println!("{}", error);
                        assert!(false)
                    }
                }
            },
            Err(error) => {
                println!("Failed to open queue: {}", error);
                assert!(false)
            }
        };
    }
}
