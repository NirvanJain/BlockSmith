use crate::models::{
    leaderboard_model::LeaderboardModel,
};

pub struct LeaderboardService;

impl LeaderboardService {
    pub fn sort_leaderboard(
        mut entries:
            Vec<LeaderboardModel>,
    ) -> Vec<LeaderboardModel> {
        entries.sort_by(|a, b| {
            b.reputation_score
                .cmp(
                    &a.reputation_score,
                )
        });

        entries
    }

    pub fn top_contributors(
        entries:
            &[LeaderboardModel],
        limit: usize,
    ) -> Vec<LeaderboardModel> {
        entries
            .iter()
            .take(limit)
            .cloned()
            .collect()
    }
}