use crate::RateLimiter;
use std::time::{Duration,Instant};

pub struct SlidingWindowCounter {
    max_requests: usize,
    window_size: Duration,
    previous_counter: usize,
    current_counter: usize,
    current_window_start: Instant
}

impl RateLimiter for SlidingWindowCounter {
    
    fn check(&mut self) -> bool {
        let now = Instant::now();
        while now > self.current_window_start + self.window_size {
            self.current_window_start = self.current_window_start + self.window_size;
            self.previous_counter = self.current_counter;
            self.current_counter = 0;
        }
        if self.current_counter + ((self.previous_counter as f64 * (1.0 - (now.duration_since(self.current_window_start).as_secs_f64()/(self.window_size.as_secs_f64())))) as usize) < self.max_requests {
            self.current_counter += 1;
            return true;
        }
        return false;
    }

    fn get_counter(&self) -> usize {
        return self.current_counter + ((self.previous_counter as f64 * (1.0 - (Instant::now().duration_since(self.current_window_start).as_secs_f64()/(self.window_size.as_secs_f64())))) as usize);
    }
}

impl SlidingWindowCounter {
    pub fn new(max_requests: usize, window_size: Duration) -> Self {
        Self {
            max_requests,
            window_size,
            previous_counter: 0,
            current_counter: 0,
            current_window_start: Instant::now(),
        }
    }
}
