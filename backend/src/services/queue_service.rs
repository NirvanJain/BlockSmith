use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct QueueService {
    queue:
        Arc<Mutex<VecDeque<String>>>,
}

impl Default for QueueService {
    fn default() -> Self {
        Self::new()
    }
}

impl QueueService {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(
                Mutex::new(
                    VecDeque::new(),
                ),
            ),
        }
    }

    pub fn enqueue(
        &self,
        job: String,
    ) {
        let mut queue =
            self.queue.lock().unwrap();

        queue.push_back(job);
    }

    pub fn dequeue(
        &self,
    ) -> Option<String> {
        let mut queue =
            self.queue.lock().unwrap();

        queue.pop_front()
    }

    pub fn size(
        &self,
    ) -> usize {
        let queue =
            self.queue.lock().unwrap();

        queue.len()
    }
}