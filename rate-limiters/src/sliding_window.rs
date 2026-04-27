use crate::RateLimiter;
use std::time::{Duration,Instant};
use std::collections::VecDeque;

pub struct SlidingWindow {
    max_requests: usize,
    timestamps: VecDeque<Instant>,
    window_size: Duration,
}

impl RateLimiter for SlidingWindow {
    fn check(&mut self) -> bool {
        let cur_timestamp = Instant::now();
        while let Some(&t) = self.timestamps.front() {
            if t < cur_timestamp - self.window_size {
                self.timestamps.pop_front();
            } else {
                break;
            }
        }

        self.timestamps.push_back(cur_timestamp);
        if self.timestamps.len() <= self.max_requests {
            return true;
        }
        return false;
    }

    fn get_counter(&self) -> usize {
        return self.timestamps.len();
    }
}

impl SlidingWindow {
    pub fn new(max_requests: usize, window_size: Duration) -> Self {
        Self {
            max_requests,
            timestamps: VecDeque::new(),
            window_size
        }
    }
}