use crate::RateLimiter;
use std::time::{Duration,Instant};
pub struct TokenBucket {
    tokens: usize,
    bucket_size: usize,
    refill_time: Duration,
    last_check: Instant,
}

impl RateLimiter for TokenBucket {
    fn check(&mut self) -> bool {
        let now = Instant::now();
        if now - self.refill_time > self.last_check {
            self.tokens = self.bucket_size - 1;
            self.last_check = now;
            return true;
        }
        if self.tokens > 0 {
            self.tokens -= 1;
            return true;
        }
        return false;
    }

    fn get_counter(&self) -> usize {
        return self.bucket_size - self.tokens;
    }
}

impl TokenBucket {
    pub fn new(bucket_size: usize, refill_time: Duration) -> Self {
        Self {
            bucket_size,
            tokens: bucket_size,
            refill_time,
            last_check: Instant::now()
        }
    }
}
