use tokio::time::{
    sleep,
    Duration,
};

pub async fn retry_operation<F, Fut, T, E>(
    mut operation: F,
    retries: usize,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<
        Output = Result<T, E>,
    >,
{
    let mut attempts = 0;

    loop {
        match operation().await {
            Ok(result) => {
                return Ok(result);
            }

            Err(error) => {
                attempts += 1;

                if attempts >= retries {
                    return Err(error);
                }

                sleep(
                    Duration::from_secs(1),
                )
                .await;
            }
        }
    }
}