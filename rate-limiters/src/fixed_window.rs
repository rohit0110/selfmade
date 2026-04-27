use std::time::{Duration,Instant};
use crate::RateLimiter;

pub struct FixedWindow {
    max_requests: u64,
    window_size: Duration,
    window_start: Instant,
    counter: u64,
}

impl RateLimiter for FixedWindow {
    fn check(&mut self) -> bool {
        while Instant::now() - self.window_start > self.window_size {
            self.window_start = self.window_start + self.window_size;
            self.counter=0;
        }
        if self.counter < self.max_requests {
            self.counter += 1;
            return true;
        } else {
            return false;
        }
    }
}

impl FixedWindow {
    pub fn new(max_requests: u64, window_size:Duration) -> Self {
        Self {
            max_requests,
            window_size,
            window_start: Instant::now(),
            counter: 0
        }
    }

    pub fn get_counter(&self) -> u64 {
        return self.counter;
    }
}