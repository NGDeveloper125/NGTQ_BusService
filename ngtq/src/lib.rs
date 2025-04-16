use std::sync::{Arc, Mutex};
pub use ngtq_error::{ NGTQError, NGTQErrorType };
mod ngtq_error;

pub trait NGTQ {
    fn initialise() -> Arc<Mutex<Self>> where Self: Sized;

    fn get_id_queue_len(&self) -> Result<usize, NGTQError>;

    fn get_category_queue_len(&self, category: &str) -> Result<usize, NGTQError>;

    fn push_id_task_to_queue(&mut self, payload: String) -> Result<String, NGTQError>;

    fn push_category_task_to_queue(&mut self, category: String, payload: String) -> Result<(), NGTQError>;

    fn pull_id_task_from_queue(&mut self, id: &str) -> Result<String, NGTQError>;

    fn pull_category_task_from_queue(&mut self, category: &str) -> Result<String, NGTQError>;
}

