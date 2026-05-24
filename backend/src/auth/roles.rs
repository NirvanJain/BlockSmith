use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
)]
pub enum UserRole {
    Admin,
    Maintainer,
    Contributor,
    Viewer,
}