pub fn calculate_reputation(
    contribution_type: &str,
) -> i32 {
    match contribution_type {
        "pull_request" => 10,

        "issue" => 5,

        "commit" => 3,

        _ => 1,
    }
}

pub fn calculate_total_score(
    contributions: Vec<String>,
) -> i32 {
    contributions
        .iter()
        .map(|contribution| {
            calculate_reputation(
                contribution,
            )
        })
        .sum()
}