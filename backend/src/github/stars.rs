use serde::{
    Deserialize,
    Serialize,
};

use crate::github::repositories::Repository;

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct RepositoryStars {
    pub repository: String,
    pub stars: u64,
}

pub fn extract_stars(
    repository: &Repository,
) -> RepositoryStars {
    RepositoryStars {
        repository:
            repository.full_name.clone(),

        stars:
            repository.stargazers_count,
    }
}