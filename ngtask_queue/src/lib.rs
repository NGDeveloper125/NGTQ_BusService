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
}
