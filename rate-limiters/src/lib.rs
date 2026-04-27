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
    fn basic_test() {
        let mut fw = FixedWindow::new(5,Duration::from_secs(60));
        for _ in 0..5{
            assert_eq!(fw.check(), true);
        }
        assert_eq!(fw.check(), false);
    }
}