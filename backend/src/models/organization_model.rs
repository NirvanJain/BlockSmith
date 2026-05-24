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
pub struct OrganizationModel {
    pub id: i64,
    pub github_org_id: i64,
    pub login: String,
    pub avatar_url: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
}