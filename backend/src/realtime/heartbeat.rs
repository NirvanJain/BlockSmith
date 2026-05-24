use tokio::time::{
    sleep,
    Duration,
};

pub async fn heartbeat_loop() {
    loop {
        println!(
            "Heartbeat ping sent"
        );

        sleep(Duration::from_secs(30))
            .await;
    }
}