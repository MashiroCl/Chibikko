use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub key_counter: Arc<Mutex<HashMap<String, u64>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            key_counter: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
