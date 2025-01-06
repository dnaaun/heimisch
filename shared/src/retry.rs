use std::fmt::Debug;

/// Will panic if passed a non-positive `n`.
pub async fn try_n_times<T, E: Debug>(
    func: impl AsyncFn() -> Result<T, E>,
    mut n: usize,
) -> Result<T, E> {
    if n == 0 {
        panic!("try_n_times() received a non-positive integer.");
    }
    loop {
        match func().await {
            Ok(t) => return Ok(t),
            Err(e) => {
                n -= 1;
                if n == 0 {
                    return Err(e);
                }
                tracing::debug!("Failed with {e:?}. Will retry {n} more times..")
            }
        }
    }
}
