use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Debug,
    Serialize,
    Deserialize,
)]
pub struct LeaderboardEntryDto {
    pub username: String,
    pub reputation_score: i32,
    pub verified_contributions:
        i32,
    pub rank: usize,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
)]
pub struct LeaderboardResponseDto {
    pub total_users: usize,
    pub leaderboard:
        Vec<LeaderboardEntryDto>,
}