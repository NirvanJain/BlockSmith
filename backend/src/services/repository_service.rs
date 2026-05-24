use crate::models::{
    repository_model::RepositoryModel,
};

pub struct RepositoryService;

impl RepositoryService {
    pub fn repository_summary(
        repository:
            &RepositoryModel,
    ) -> String {
        format!(
            "{} has {} stars and {} forks",
            repository.full_name,
            repository.stars,
            repository.forks
        )
    }

    pub fn is_popular(
        repository:
            &RepositoryModel,
    ) -> bool {
        repository.stars > 100
    }
}