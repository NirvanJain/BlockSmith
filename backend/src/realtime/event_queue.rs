use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct EventQueue {
    queue:
        Arc<Mutex<VecDeque<String>>>,
}

impl EventQueue {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(
                Mutex::new(
                    VecDeque::new(),
                ),
            ),
        }
    }

    pub fn push(
        &self,
        event: String,
    ) {
        let mut queue =
            self.queue.lock().unwrap();

        queue.push_back(event);
    }

    pub fn pop(
        &self,
    ) -> Option<String> {
        let mut queue =
            self.queue.lock().unwrap();

        queue.pop_front()
    }

    pub fn len(
        &self,
    ) -> usize {
        let queue =
            self.queue.lock().unwrap();

        queue.len()
    }
}