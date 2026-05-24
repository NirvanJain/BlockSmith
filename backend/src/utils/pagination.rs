use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub page: usize,
    pub per_page: usize,
}

impl Pagination {
    pub fn offset(&self) -> usize {
        (self.page - 1) * self.per_page
    }
}