use std::{collections::HashMap, sync::{Arc, Mutex}};

use serde::{Deserialize, Serialize};


#[derive(Debug)]
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

    pub fn pull_category_task(&mut self, category: String) -> Option<String> {
        match self.category_queues.remove(&category) {
            Some(mut queue) => {
                if queue.len() > 1 {
                    let payload = queue.remove(0);
                    self.category_queues.insert(category.to_string(), queue);
                    Some(payload)
                } else {
                    Some(queue.remove(0))
                }
            },
            None => None
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
}
