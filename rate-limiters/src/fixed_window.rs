use std::time::{Duration,Instant};
use crate::RateLimiter;

pub struct FixedWindow {
    max_requests: usize,
    window_size: Duration,
    window_start: Instant,
    counter: usize,
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

    fn get_counter(&self) -> usize {
        return self.counter;
    }
}

impl FixedWindow {
    pub fn new(max_requests: usize, window_size:Duration) -> Self {
        Self {
            max_requests,
            window_size,
            window_start: Instant::now(),
            counter: 0
        }
    }
}