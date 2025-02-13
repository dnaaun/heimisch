use std::{fmt::Debug, future::Future};

/// # Panics
///
/// Panics if `n` = 0.
pub async fn try_n_times<Fut, T, E>(func: impl Fn() -> Fut, mut n: usize) -> Result<T, E>
where
    Fut: Future<Output = Result<T, E>>,
    E: Debug,
{
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
