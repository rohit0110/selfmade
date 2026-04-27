pub trait RateLimiter {
    fn check(&mut self) -> bool;
}

mod fixed_window;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration};
    use fixed_window::FixedWindow;

    #[test]
    fn fails_after_max_req_reached() {
        let mut fw = FixedWindow::new(5,Duration::from_secs(60));
        for _ in 0..5{
            assert_eq!(fw.check(), true);
        }
        assert_eq!(fw.check(), false);
    }

    #[test]
    fn resets_counter_after_time_window() {
        let mut fw = FixedWindow::new(5,Duration::from_millis(100));
        for _ in 0..3{
            assert_eq!(fw.check(),true);
        }
        assert_eq!(fw.get_counter(), 3);
        std::thread::sleep(Duration::from_millis(150));
        fw.check();
        assert_eq!(fw.get_counter(), 1);
    }
}