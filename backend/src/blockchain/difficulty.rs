pub fn calculate_difficulty(
    total_blocks: usize,
) -> usize {
    match total_blocks {
        0..=10 => 2,

        11..=50 => 3,

        51..=100 => 4,

        _ => 5,
    }
}