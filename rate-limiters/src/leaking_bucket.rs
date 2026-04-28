use crate::RateLimiter;
use std::time::{Duration, Instant};

pub struct LeakingBucket {
    bucket_size: usize,
    bucket_level: usize ,
    last_check: Instant,
    leak_rate: Duration
}

impl RateLimiter for LeakingBucket {
    fn check(&mut self) -> bool {
        let now = Instant::now();
        let time_passed = now - self.last_check;
        let mut req_passed = (time_passed.as_secs_f64()/self.leak_rate.as_secs_f64()).floor() as usize;
        if req_passed > self.bucket_level {
            req_passed = self.bucket_level;
        }
        self.bucket_level -= req_passed;
        self.last_check += self.leak_rate * req_passed as u32 ;
        if self.bucket_level == self.bucket_size {
            return false;
        }
        self.bucket_level += 1;
        return true;
    }

    fn get_counter(&self) -> usize {
        return self.bucket_level;
    }
}

impl LeakingBucket {
    pub fn new(bucket_size: usize, leak_rate: Duration) -> Self {
        Self{
            bucket_size,
            bucket_level: 0,
            leak_rate,
            last_check: Instant::now()
        }
    }
}