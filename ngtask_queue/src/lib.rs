use std::{collections::HashMap, sync::{Arc, Mutex}};
use ngtq::{ NGCategoryTask, NGIdTask, NGTQ };
use serde::{Deserialize, Serialize};

pub struct TaskQueue {
    pub is_initialised: bool,
    pub id_queue: HashMap<String, String>,
    pub category_queues: HashMap<String, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdTask {
    pub id: String,
    pub payload: String,
}

impl NGIdTask for IdTask {
    fn get_id(&self) -> &str {
        &self.id
    }
    
    fn get_payload(&self) -> String {
        self.payload.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryTask {
    pub category: String,
    pub payload: String,
}

impl NGCategoryTask for CategoryTask {  
    fn get_category(&self) -> &str {
        &self.category
    }
    
    fn get_payload(&self) -> String {
        self.payload.to_string()
    }
}

impl NGTQ for TaskQueue {
    fn initialise() -> Arc<Mutex<TaskQueue>> {
        let is_initialised = true;
        let id_queue: HashMap<String, String> = HashMap::new();
        let category_queues: HashMap<String, Vec<String>> = HashMap::new();

        Arc::new(Mutex::new(TaskQueue { is_initialised, id_queue, category_queues }))
    }

    fn get_id_queue_len(&self) -> Result<usize, String> {
        if !self.is_initialised {
            return Err(String::from("Failed to get queue length - The TaskQueue was not initialised"))
        }
        Ok(self.id_queue.len())
    }

    fn get_category_queue_len(&self, category: &str) -> Result<usize, String> {
        if !self.is_initialised {
            return Err(String::from("Failed to get queue length - The TaskQueue was not initialised"))
        }
        match self.category_queues.get(category) {
            Some(queue) => return  Ok(queue.len()),
            None => return Err(String::from("Failed to get queue length - No queue found for this category"))
        }
    }



    fn push_id_task_to_queue<T: NGIdTask>(&mut self, task: T) -> Result<(), String> {
        if !self.is_initialised {
            return Err(String::from("Failed to push new task - The TaskQueue was not initialised"))
        }

        if task.get_id() == String::new() || task.get_payload() == String::new() {
            return Err(String::from("Failed to push new task - The task id or payload is empty"))
        } else if self.id_queue.contains_key(task.get_id()) {
            return Err(String::from("Failed to push new task - A task with this id already exist in the queue"))    
        } else {
            return match self.id_queue.insert(task.get_id().to_string(), task.get_payload()) {
                Some(_) => return Err(String::from("Fatal Error - A task with this id was in the queuea")), 
                None => Ok(())
            }
        }
    }
    
    fn push_category_task_to_queue<T: NGCategoryTask>(&mut self, task: T) -> Result<(), String> {
        if !self.is_initialised {
            return Err(String::from("Failed to push new task - The TaskQueue was not initialised"))
        }

        if task.get_category() == String::new() || task.get_payload() == String::new() {
            return Err(String::from("Failed to push new task - The task topic or payload is empty"))
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
    
    fn pull_id_task_from_queue(&mut self, id: String) -> Result<String, String> {
        if !self.is_initialised {
            return Err(String::from("Failed to pull task - The TaskQueue was not initialised"))
        }
        match self.id_queue.remove(&id) {
            Some(payload) => Ok(payload),
            None => Err(String::from("Failed to pull task from queue - task with this id was not found in queue"))
        }
    }
    
    fn pull_category_task_from_queue(&mut self, category: String) -> Result<String, String> {
        if !self.is_initialised {
            return Err(String::from("Failed to pull task - The TaskQueue was not initialised"))
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
            None => Err(String::from("Failed to pull task from queue - no tasks for this topic were found"))
        }
    }
}

// fn get_category_queue<'a>(task_queue: &'a TaskQueue, category: &'a str) -> Option<&'a Vec<String>> {
//     if !task_queue.is_initialised {
//         return None
//     }
//     task_queue.category_queues.get(category)
// }

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
        let task = IdTask {
            id: String::from("test"),
            payload: String::from("Do Somthing")
        };

        match task_queue_arc.lock() {
            Ok(mut task_queue) => {
                assert_eq!(task_queue.get_id_queue_len().unwrap(), 0);
                match task_queue.push_id_task_to_queue(task) {
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
