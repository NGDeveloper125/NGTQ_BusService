use std::{collections::HashMap, sync::{Arc, Mutex}};

use serde::{Deserialize, Serialize};


#[derive(Debug)]
pub struct TaskQueue {
    is_initialised: bool,
    id_queue: HashMap<String, String>,
    category_queues: HashMap<String, Mutex<Vec<String>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdTask {
    id: String,
    payload: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryTask {
    category: String,
    payload: String,
}

impl TaskQueue {
    pub fn initialise() -> Arc<Mutex<TaskQueue>> {
        let is_initialised = true;
        let id_queue: HashMap<String, String> = HashMap::new();
        let category_queues: HashMap<String, Mutex<Vec<String>>> = HashMap::new();

        Arc::new(Mutex::new(TaskQueue { is_initialised, id_queue, category_queues }))
    }

    pub fn push_id_task_to_queue(&mut self, task: IdTask) -> usize {
        if task.id == String::new() || task.payload == String::new() || self.id_queue.contains_key(&task.id) {
            0
        } else {
            match self.id_queue.insert(task.id, task.payload) {
                Some(_) => 0, // should never happen - handle as an error
                None => self.id_queue.len()
            }
        }
    }
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
    fn valid_new_message_test_push_id_task_to_queue() {
        let task_queue_arc = TaskQueue::initialise();
        let id_task = IdTask {
            id: String::from("1"),
            payload: String::from("Do this and that")
        };

        match task_queue_arc.lock() {
            Ok(mut task_queue) => {
                let i = task_queue.push_id_task_to_queue(id_task);
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
                let i = task_queue.push_id_task_to_queue(id_task1);
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
                let i = task_queue.push_id_task_to_queue(id_task2);
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
                let i = task_queue.push_id_task_to_queue(id_task);
                assert_eq!(i, 0);
                assert_eq!(task_queue.id_queue.len(), 0)
            },
            Err(error) => {
                println!("Failed to open queue: {:?}", error);
                assert!(false)
            }
        };
    }
}
