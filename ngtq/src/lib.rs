use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
pub use ngtq_error::{ NGTQError, NGTQErrorType };
mod ngtq_error;

pub trait NGIdTask: Serialize + for<'de> Deserialize<'de>  {
    fn get_id(&self) -> &str;
    fn get_payload(&self) -> String;
}

pub trait NGCategoryTask: Serialize + for<'de> Deserialize<'de> {
    fn get_category(&self) -> &str;
    fn get_payload(&self) -> String;
}

pub trait NGTQ {
    fn initialise() -> Arc<Mutex<Self>> where Self: Sized;

    fn get_id_queue_len(&self) -> Result<usize, NGTQError>;

    fn get_category_queue_len(&self, category: &str) -> Result<usize, NGTQError>;

    fn push_id_task_to_queue<A>(&mut self, task: A) -> Result<(), NGTQError> where A: NGIdTask;

    fn push_category_task_to_queue<B>(&mut self, task: B) -> Result<(), NGTQError> where B: NGCategoryTask;

    fn pull_id_task_from_queue(&mut self, id: String) -> Result<String, NGTQError>;

    fn pull_category_task_from_queue(&mut self, category: String) -> Result<String, NGTQError>;
}

