//! Time-related test helpers for timeouts and eventual consistency

use std::future::Future;
use std::time::Duration;
use tokio::time::{error::Elapsed, timeout};

/// Execute a future with a timeout.
///
/// This is a convenience wrapper around `tokio::time::timeout` that makes
/// test timeouts more explicit and easier to adjust globally if needed.
pub async fn with_timeout<T>(
    duration: Duration,
    future: impl Future<Output = T>,
) -> Result<T, Elapsed> {
    timeout(duration, future).await
}

/// Execute a future with a default test timeout of 5 seconds.
pub async fn with_default_timeout<T>(future: impl Future<Output = T>) -> Result<T, Elapsed> {
    with_timeout(Duration::from_secs(5), future).await
}

/// Wait until a condition becomes true, checking periodically.
///
/// This is useful for testing eventually consistent behavior or waiting
/// for async operations to complete.
///
/// # Arguments
/// * `max_duration` - Maximum time to wait before giving up
/// * `poll_interval` - How often to check the condition
/// * `condition` - A closure that returns true when the condition is met
///
/// # Returns
/// `true` if the condition was met within the timeout, `false` otherwise
pub async fn wait_until<F>(
    max_duration: Duration,
    poll_interval: Duration,
    mut condition: F,
) -> bool
where
    F: FnMut() -> bool,
{
    let deadline = tokio::time::Instant::now() + max_duration;

    while tokio::time::Instant::now() < deadline {
        if condition() {
            return true;
        }
        tokio::time::sleep(poll_interval).await;
    }

    // Check one more time at the deadline
    condition()
}

/// Wait until an async condition becomes true, checking periodically.
///
/// Similar to `wait_until` but for async conditions.
pub async fn wait_until_async<F, Fut>(
    max_duration: Duration,
    poll_interval: Duration,
    mut condition: F,
) -> bool
where
    F: FnMut() -> Fut,
    Fut: Future<Output = bool>,
{
    let deadline = tokio::time::Instant::now() + max_duration;

    while tokio::time::Instant::now() < deadline {
        if condition().await {
            return true;
        }
        tokio::time::sleep(poll_interval).await;
    }

    // Check one more time at the deadline
    condition().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_with_timeout_success() {
        let result = with_timeout(Duration::from_secs(1), async { 42 }).await;
        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_with_timeout_failure() {
        let result = with_timeout(Duration::from_millis(10), async {
            tokio::time::sleep(Duration::from_secs(1)).await;
            42
        })
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_wait_until() {
        let flag = Arc::new(AtomicBool::new(false));
        let flag_clone = flag.clone();

        // Set flag after 50ms
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(50)).await;
            flag_clone.store(true, Ordering::SeqCst);
        });

        // Wait for flag with 100ms timeout
        let result = wait_until(
            Duration::from_millis(100),
            Duration::from_millis(10),
            || flag.load(Ordering::SeqCst),
        )
        .await;

        assert!(result);
    }
}
