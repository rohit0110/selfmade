pub trait RateLimiter {
    fn check(&mut self) -> bool;
    fn get_counter(&self) -> usize;
}

pub mod fixed_window;
pub mod sliding_window;
pub mod sliding_window_counter;
pub mod leaking_bucket;
pub mod token_bucket;

#[cfg(test)]
mod tests {
    use crate::{leaking_bucket::LeakingBucket, sliding_window::SlidingWindow, token_bucket::TokenBucket};

    use super::*;
    use std::time::{Duration};
    use std::thread;
    use std::sync::{Arc, Mutex};
    use fixed_window::FixedWindow;
    use sliding_window_counter::SlidingWindowCounter;

    fn fails_after_max_req_reached(limiter: &mut dyn RateLimiter) {
        for _ in 0..5{
            assert_eq!(limiter.check(), true);
        }
        assert_eq!(limiter.check(), false);
    }

    fn resets_counter_after_time_window(limiter: &mut dyn RateLimiter) {
        for _ in 0..3{
            assert_eq!(limiter.check(),true);
        }
        assert_eq!(limiter.get_counter(), 3);
        std::thread::sleep(Duration::from_millis(350));
        limiter.check();
        assert_eq!(limiter.get_counter(), 1);
    }

    #[test]
    fn fixed_window_test() {
        let mut fw = FixedWindow::new(5,Duration::from_millis(100));
        fails_after_max_req_reached(&mut fw);
        fw = FixedWindow::new(5,Duration::from_millis(100));
        resets_counter_after_time_window(&mut fw);
    }

    #[test]
    fn sliding_window_test() {
        let mut sw = SlidingWindow::new(5, Duration::from_millis(100));
        fails_after_max_req_reached(&mut sw);
        sw = SlidingWindow::new(5, Duration::from_millis(100));
        resets_counter_after_time_window(&mut sw);
    }

    #[test]
    fn sliding_window_counter_test() {
        let mut swc = SlidingWindowCounter::new(5,Duration::from_millis(100));
        fails_after_max_req_reached(&mut swc);
        swc = SlidingWindowCounter::new(5,Duration::from_millis(100));
        resets_counter_after_time_window(&mut swc);
    }

    #[test]
    fn leaking_bucket_test() {
        let mut lb = LeakingBucket::new(5, Duration::from_millis(100));
        fails_after_max_req_reached(&mut lb);
        lb = LeakingBucket::new(5, Duration::from_millis(100));
        resets_counter_after_time_window(&mut lb);
    }

    #[test]
    fn token_bucket_test() {
        let mut tb = TokenBucket::new(5, Duration::from_millis(100));
        fails_after_max_req_reached(&mut tb);
        tb = TokenBucket::new(5, Duration::from_millis(100));
        resets_counter_after_time_window(&mut tb);
    }

    #[test]
    fn concurrency_test() {
        let limiter = Arc::new(Mutex::new(TokenBucket::new(5, Duration::from_millis(100))));
        let mut handles = vec![];

        for i in 0..10 {
            let clone = limiter.clone();
            let handle = thread::spawn(move || {
                for _ in 0..20{
                    thread::sleep(Duration::from_millis(20));
                    let result = clone.lock().unwrap().check();
                    println!("THREAD {}, {}", i, result);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}


