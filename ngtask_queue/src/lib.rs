use std::{collections::HashMap, sync::{Arc, Mutex}};

use serde::{Deserialize, Serialize};


#[derive(Debug)]
pub struct TaskQueue {
    is_initialised: bool,
    id_queue: HashMap<String, String>,
    category_queues: HashMap<String, Vec<String>>,
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
        let category_queues: HashMap<String, Vec<String>> = HashMap::new();

        Arc::new(Mutex::new(TaskQueue { is_initialised, id_queue, category_queues }))
    }

    pub fn push_id_task(&mut self, task: IdTask) -> usize {
        if task.id == String::new() || task.payload == String::new() || self.id_queue.contains_key(&task.id) {
            0
        } else {
            match self.id_queue.insert(task.id, task.payload) {
                Some(_) => 0, // should never happen - handle as an error
                None => self.id_queue.len()
            }
        }
    }

    pub fn push_category_task(&mut self, task: CategoryTask) -> (usize, usize) {
        if task.category == String::new() || task.payload == String::new() {
            return (0,0)
        } 
        match self.category_queues.get_mut(&task.category) {
            Some(queue) => {
                let queue_size = push_category_task_to_existing_queue(queue, task);
                (self.category_queues.len(), queue_size)
            },
            None => {
                let mut new_queue = Vec::new();
                new_queue.push(task.payload);
                self.category_queues.insert(task.category.to_string(), new_queue);
                return (self.category_queues.len(), 1);
            }
        }
    }

    pub fn pull_id_task(&mut self, id: String) -> Option<String> {
        self.id_queue.remove(&id)
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

    #[test]
    fn valid_new_message_test_push_new_category_task_to_queue() {
        let task_queue_arc = TaskQueue::initialise();
        let task = CategoryTask {
            category: String::from("test"),
            payload: String::from("Do this and that")
        };

        match task_queue_arc.lock() {
            Ok(mut task_queue) => {
                let (number_of_queue, queue_len) = task_queue.push_category_task(task);
                assert_eq!(number_of_queue, 1);
                assert_eq!(queue_len, 1);
            },
            Err(error) => {
                println!("Failed to open queue: {:?}", error);
                assert!(false)
            }
        };
    }

    #[test]
    fn valid_new_message_test_push_existing_category_task_to_queue() {
        let task_queue_arc = TaskQueue::initialise();
        let task = CategoryTask {
            category: String::from("test"),
            payload: String::from("Do this and that")
        };

        match task_queue_arc.lock() {
            Ok(mut task_queue) => {
                let (number_of_queue, queue_len) = task_queue.push_category_task(task);
                assert_eq!(number_of_queue, 1);
                assert_eq!(queue_len, 1);
            },
            Err(error) => {
                println!("Failed to open queue: {:?}", error);
                assert!(false)
            }
        };

        let task = CategoryTask {
            category: String::from("test"),
            payload: String::from("Do this and that")
        };

        match task_queue_arc.lock() {
            Ok(mut task_queue) => {
                let (number_of_queue, queue_len) = task_queue.push_category_task(task);
                assert_eq!(number_of_queue, 1);
                assert_eq!(queue_len, 2);
            },
            Err(error) => {
                println!("Failed to open queue: {:?}", error);
                assert!(false)
            }
        };
    }

    #[test]
    fn invalid_category_new_message_test_push_category_task_to_queue() {
        let task_queue_arc = TaskQueue::initialise();
        let task = CategoryTask {
            category: String::new(),
            payload: String::from("Do this and that")
        };

        match task_queue_arc.lock() {
            Ok(mut task_queue) => {
                let (number_of_queue, queue_len) = task_queue.push_category_task(task);
                assert_eq!(number_of_queue, 0);
                assert_eq!(queue_len, 0);
            },
            Err(error) => {
                println!("Failed to open queue: {:?}", error);
                assert!(false)
            }
        };
    }

    #[test]
    fn invalid_payload_new_message_test_push_category_task_to_queue() {
        let task_queue_arc = TaskQueue::initialise();
        let task = CategoryTask {
            category: String::from("test"),
            payload: String::new()
        };

        match task_queue_arc.lock() {
            Ok(mut task_queue) => {
                let (number_of_queue, queue_len) = task_queue.push_category_task(task);
                assert_eq!(number_of_queue, 0);
                assert_eq!(queue_len, 0);
            },
            Err(error) => {
                println!("Failed to open queue: {:?}", error);
                assert!(false)
            }
        };
    }

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
                        assert_eq!(task_queue.id_queue.len(), 0)
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
}
