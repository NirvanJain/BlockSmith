use tokio::time::{
    sleep,
    Duration,
};

pub async fn stream_latest_blocks(
) {
    loop {
        println!(
            "Streaming latest blockchain blocks..."
        );

        sleep(Duration::from_secs(5))
            .await;
    }
}