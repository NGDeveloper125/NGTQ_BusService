use std::{collections::HashMap, sync::{Arc, Mutex}};

use serde::{Deserialize, Serialize};


#[derive(Debug)]
pub struct TaskQueue {
    pub is_initialised: bool,
    id_queue: HashMap<String, String>,
    category_queues: HashMap<String, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdTask {
    pub id: String,
    pub payload: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryTask {
    pub category: String,
    pub payload: String,
}

impl TaskQueue {
    pub fn initialise() -> Arc<Mutex<TaskQueue>> {
        let is_initialised = true;
        let id_queue: HashMap<String, String> = HashMap::new();
        let category_queues: HashMap<String, Vec<String>> = HashMap::new();

        Arc::new(Mutex::new(TaskQueue { is_initialised, id_queue, category_queues }))
    }

    pub fn get_id_queue_len(&self) -> Option<usize> {
        if !self.is_initialised {
            return None
        }
        Some(self.id_queue.len())
    }

    pub fn get_category_queue_len(&self, category: &str) -> Option<usize> {
        if !self.is_initialised {
            return None
        }
        match self.category_queues.get(category) {
            Some(queue) => return  Some(queue.len()),
            None => return Some(0)
        }
    }

    pub fn get_category_queue(&self, category: &str) -> Option<&Vec<String>> {
        if !self.is_initialised {
            return None
        }
        self.category_queues.get(category)
    }

    pub fn push_id_task(&mut self, task: IdTask) -> Result<usize, String> {
        if !self.is_initialised {
            return Err(String::from("Failed to push new task - The TaskQueue was not initialised"))
        }

        if task.id == String::new() || task.payload == String::new() {
            return Err(String::from("Failed to push new task - The task id or payload is empty"))
        } else if self.id_queue.contains_key(&task.id) {
            return Err(String::from("Failed to push new task - A task with this id already exist in the queue"))    
        } else {
            return match self.id_queue.insert(task.id, task.payload) {
                Some(_) => return Err(String::from("Fatal Error - A task with this id was in the queuea")), 
                None => Ok(self.id_queue.len())
            }
        }
    }
    

    pub fn push_category_task(&mut self, task: CategoryTask) -> Result<(usize, usize), String> {
        if !self.is_initialised {
            return Err(String::from("Failed to push new task - The TaskQueue was not initialised"))
        }

        if task.category == String::new() || task.payload == String::new() {
            return Err(String::from("Failed to push new task - The task topic or payload is empty"))
        } 
        match self.category_queues.get_mut(&task.category) {
            Some(queue) => {
                let queue_size = push_category_task_to_existing_queue(queue, task);
                return Ok((self.category_queues.len(), queue_size))
            },
            None => {
                let mut new_queue = Vec::new();
                new_queue.push(task.payload);
                self.category_queues.insert(task.category.to_string(), new_queue);
                return Ok((self.category_queues.len(), 1));
            }
        }
    }

    pub fn pull_id_task(&mut self, id: String) -> Result<String, String> {
        if !self.is_initialised {
            return Err(String::from("Failed to pull task - The TaskQueue was not initialised"))
        }
        match self.id_queue.remove(&id) {
            Some(payload) => Ok(payload),
            None => Err(String::from("Failed to pull task from queue - task with this id was not found in queue"))
        }
    }

    pub fn pull_category_task(&mut self, category: String) -> Result<String, String> {
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


fn push_category_task_to_existing_queue(queue: &mut Vec<String>, task: CategoryTask) -> usize {
    queue.push(task.payload);
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
                match task_queue.push_id_task(task) {
                    Ok(_) => assert_eq!(task_queue.get_id_queue_len().unwrap(), 1),
                    Err(error) => {
                        println!("Failed to psuh task to queue: {}", error);
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
                assert_eq!(task_queue.get_category_queue_len(&category).unwrap(), 0);
                task_queue.push_category_task(task);
                assert_eq!(task_queue.get_category_queue_len(&category).unwrap(), 1);
            },
            Err(error) => {
                println!("Failed to open queue: {}", error);
                assert!(false)
            }
        };
    }
}
