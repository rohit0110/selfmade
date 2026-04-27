pub trait RateLimiter {
    fn check(&mut self) -> bool;
    fn get_counter(&self) -> usize;
}

mod fixed_window;
mod sliding_window;

#[cfg(test)]
mod tests {
    use crate::sliding_window::SlidingWindow;

    use super::*;
    use std::time::{Duration};
    use fixed_window::FixedWindow;

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
        std::thread::sleep(Duration::from_millis(150));
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
}