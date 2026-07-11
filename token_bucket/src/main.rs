use std::time::Instant;
use tracing::trace;

struct TokenBucket {
    /// maximum number of token
    capacity: u64,
    /// current number of token
    tokens: u64,
    /// refill rate: 4/min
    refill_rate: u64,
    /// last fill
    last_refill: Instant,
}

impl TokenBucket {
    /// start the new bucket with full token capacity
    fn new(capacity: u64, refill_rate: u64) -> Self {
        trace!(
            "Creating a new token bucket with capacity: {} & refill_rate: {}",
            capacity,
            refill_rate
        );

        TokenBucket {
            capacity,
            tokens: capacity,
            refill_rate,
            last_refill: Instant::now(),
        }
    }

    /// Refills the bucket based on the elapsed time since the last refill.
    ///
    /// Adds tokens to the bucket based on the `refill_rate` and the amount of
    /// time that has passed since the last refill. It ensures the bucket does
    /// not exceed the defined `capacity`.
    ///
    /// This function runs synchronously, but is called asynchronously in the context of `try_consume`.
    async fn refill(&mut self) {
        let now = Instant::now();
        let time_since_last_refill = now.duration_since(self.last_refill).as_secs();

        if time_since_last_refill > 0 {
            let tokens_to_fill = time_since_last_refill * self.refill_rate;
            trace!(
                "Refilling bucket: adding {} tokens, after {} secs",
                tokens_to_fill,
                time_since_last_refill
            );

            self.tokens = (self.tokens + tokens_to_fill).min(self.capacity);
            self.last_refill = now;
        } else {
            trace!("No need to refill, less than 1 sec has been passed");
        }
    }

    async fn try_consume(&mut self, amount: u64) -> bool {
        self.refill().await;

        if self.tokens >= amount {
            self.tokens -= amount;
            trace!(
                "Consumed {} tokens, {} tokens left in the bucket",
                amount,
                self.tokens
            );
            true
        } else {
            trace!(
                "Failed to consume {} tokens, only {} tokens left in the bucket",
                amount,
                self.tokens
            );
            false
        }

    }

    async fn available_tokens(&mut self) -> u64 {
        self.refill().await;
        self.tokens
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use tokio::time::{sleep, Duration};
    use crate::TokenBucket;

    #[tokio::test]
    async fn test_new_token_bucket() {
        let mut bucket = TokenBucket::new(10, 5);
        assert_eq!(bucket.available_tokens().await, 10);
    }

    #[tokio::test]
    async fn test_try_consume_success() {
        let mut bucket = TokenBucket::new(10, 5);
        assert!(bucket.try_consume(3).await);
        assert_eq!(bucket.available_tokens().await, 7);
    }

    #[tokio::test]
    async fn test_try_consume_failure() {
        let mut bucket = TokenBucket::new(10, 5);
        assert!(bucket.try_consume(11).await);
        assert_eq!(bucket.available_tokens().await, 10);
    }

    #[tokio::test]
    async fn test_refill() {
        let mut bucket = TokenBucket::new(10, 5);
        assert!(bucket.try_consume(10).await);
        assert_eq!(bucket.available_tokens().await, 0);

        sleep(Duration::from_secs(1)).await;
        assert!(bucket.try_consume(5).await);
        assert_eq!(bucket.available_tokens().await, 0);
    }

    #[tokio::test]
    async fn test_refill_up_to_capacity() {
        let mut bucket = TokenBucket::new(10, 5);
        assert!(bucket.try_consume(10).await);
        sleep(Duration::from_secs(2)).await;
        assert!(bucket.try_consume(10).await);
        sleep(Duration::from_secs(2)).await;
        assert_eq!(bucket.available_tokens().await, 10);
    }

    #[tokio::test]
    async fn test_consume_zero_tokens() {
        let mut bucket = TokenBucket::new(10, 5);
        assert!(bucket.try_consume(0).await);
        assert_eq!(bucket.available_tokens().await, 10);
    }

    #[tokio::test]
    async fn test_rapid_consume() {
        let mut bucket = TokenBucket::new(1000, 1000);
        for _ in 0..1000 {
            assert!(bucket.try_consume(1).await);
        }
        assert!(!bucket.try_consume(1).await);
    }

    #[tokio::test]
    async fn test_multiple_consume_and_refill() {
        let mut bucket = TokenBucket::new(10, 2);

        for _ in 0..5 {
            assert!(bucket.try_consume(2).await);
            sleep(Duration::from_millis(500)).await;
        }

        assert!(!bucket.try_consume(5).await);
        assert!(bucket.try_consume(4).await);
        assert!(!bucket.try_consume(1).await);
        assert_eq!(bucket.available_tokens().await, 0);

        sleep(Duration::from_secs(1)).await;
        assert!(bucket.try_consume(2).await);
        assert_eq!(bucket.available_tokens().await, 0);
    }
}
