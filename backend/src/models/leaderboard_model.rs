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
pub struct LeaderboardModel {
    pub id: i64,
    pub user_id: i64,
    pub reputation_score: i32,
    pub verified_contributions:
        i32,
    pub rank_position: i32,
    pub updated_at: String,
}