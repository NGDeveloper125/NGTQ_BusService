use std::sync::{Arc, Mutex};

pub trait NGTaskQueue {

}

pub trait NGIdTask {
    fn get_id(&self) -> &str;
    fn get_payload(&self) -> String;
}

pub trait NGCategoryTask {
    fn get_category(&self) -> &str;
    fn get_payload(&self) -> String;
}

pub trait NGTQ<T> {
    fn initialise() -> Arc<Mutex<T>> where T: NGTaskQueue;

    fn get_id_queue_len(&self) -> Result<usize, String>;

    fn get_category_queue_len(&self, category: &str) -> Result<usize, String>;

    fn push_id_task_to_queue<A>(&mut self, task: A) -> Result<(), String> where A: NGIdTask;

    fn push_category_task_to_queue<B>(&mut self, task: B) -> Result<(), String> where B: NGCategoryTask;

    fn pull_id_task_from_queue(&mut self, id: String) -> Result<String, String>;

    fn pull_category_task_from_queue(&mut self, category: String) -> Result<String, String>;
}

