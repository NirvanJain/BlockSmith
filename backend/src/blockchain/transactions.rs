use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct Transaction {
    pub contributor: String,
    pub repository: String,
    pub contribution_type: String,
    pub contribution_link: String,
    pub timestamp: String,
}

impl Transaction {
    pub fn new(
        contributor: String,
        repository: String,
        contribution_type: String,
        contribution_link: String,
        timestamp: String,
    ) -> Self {
        Self {
            contributor,
            repository,
            contribution_type,
            contribution_link,
            timestamp,
        }
    }
}