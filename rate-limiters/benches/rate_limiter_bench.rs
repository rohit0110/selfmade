use criterion::{criterion_group, criterion_main, Criterion};
use rate_limiters::token_bucket::TokenBucket;
use rate_limiters::fixed_window::FixedWindow;
use rate_limiters::leaking_bucket::LeakingBucket;
use rate_limiters::sliding_window::SlidingWindow;
use rate_limiters::sliding_window_counter::SlidingWindowCounter;
use rate_limiters::RateLimiter;
use std::thread;
use std::time::Duration;
use std::sync::{Arc,Mutex};

fn benchmark_token_bucket(c: &mut Criterion) {
    c.bench_function("token_bucket_single_thread", |b| {
        b.iter_with_setup(|| TokenBucket::new(5,Duration::from_millis(100)), |mut limiter| {
            for _ in 0..20 {
                limiter.check();
            }
        })
    } );
}

fn benchmark_token_bucket_concurrent_20(c: &mut Criterion) {
    c.bench_function("token_bucket_multiple_thread_20", |b|{
        b.iter_with_setup(|| Arc::new(Mutex::new(TokenBucket::new(100, Duration::from_millis(100)))), |limiter| {
            let mut handles = vec![];
            for _ in 0..20 {
                let clone = limiter.clone();
                let handle = thread::spawn(move || {
                        for _ in 0..25 {
                            clone.lock().unwrap().check();
                        }
                    });
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }
        })
    });
}

fn benchmark_fixed_window(c: &mut Criterion) {
    c.bench_function("fixed_window_single_thread", |b| {
        b.iter_with_setup(|| FixedWindow::new(5, Duration::from_millis(100)), |mut limiter| {
            for _ in 0..20 {
                limiter.check();
            }
        })
    });
}

fn benchmark_fixed_window_concurrent_20(c: &mut Criterion) {
    c.bench_function("fixed_window_concurrent_20", |b| {
        b.iter_with_setup(|| Arc::new(Mutex::new(FixedWindow::new(100, Duration::from_millis(100)))), |limiter| {
            let mut handles = vec![];
            for _ in 0..20 {
                let clone = limiter.clone();
                let handle = thread::spawn(move || {
                    for _ in 0..25 {
                        clone.lock().unwrap().check();
                    }
                });
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }
        })
    });
}

fn benchmark_leaking_bucket(c: &mut Criterion) {
    c.bench_function("leaking_bucket_single_thread", |b| {
        b.iter_with_setup(|| LeakingBucket::new(5, Duration::from_millis(100)), |mut limiter| {
            for _ in 0..20 {
                limiter.check();
            }
        })
    });
}

fn benchmark_leaking_bucket_concurrent_20(c: &mut Criterion) {
    c.bench_function("leaking_bucket_concurrent_20", |b| {
        b.iter_with_setup(|| Arc::new(Mutex::new(LeakingBucket::new(100, Duration::from_millis(100)))), |limiter| {
            let mut handles = vec![];
            for _ in 0..20 {
                let clone = limiter.clone();
                let handle = thread::spawn(move || {
                    for _ in 0..25 {
                        clone.lock().unwrap().check();
                    }
                });
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }
        })
    });
}

fn benchmark_sliding_window(c: &mut Criterion) {
    c.bench_function("sliding_window_single_thread", |b| {
        b.iter_with_setup(|| SlidingWindow::new(5, Duration::from_millis(100)), |mut limiter| {
            for _ in 0..20 {
                limiter.check();
            }
        })
    });
}

fn benchmark_sliding_window_concurrent_20(c: &mut Criterion) {
    c.bench_function("sliding_window_concurrent_20", |b| {
        b.iter_with_setup(|| Arc::new(Mutex::new(SlidingWindow::new(100, Duration::from_millis(100)))), |limiter| {
            let mut handles = vec![];
            for _ in 0..20 {
                let clone = limiter.clone();
                let handle = thread::spawn(move || {
                    for _ in 0..25 {
                        clone.lock().unwrap().check();
                    }
                });
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }
        })
    });
}

fn benchmark_sliding_window_counter(c: &mut Criterion) {
    c.bench_function("sliding_window_counter_single_thread", |b| {
        b.iter_with_setup(|| SlidingWindowCounter::new(5, Duration::from_millis(100)), |mut limiter| {
            for _ in 0..20 {
                limiter.check();
            }
        })
    });
}

fn benchmark_sliding_window_counter_concurrent_20(c: &mut Criterion) {
    c.bench_function("sliding_window_counter_concurrent_20", |b| {
        b.iter_with_setup(|| Arc::new(Mutex::new(SlidingWindowCounter::new(100, Duration::from_millis(100)))), |limiter| {
            let mut handles = vec![];
            for _ in 0..20 {
                let clone = limiter.clone();
                let handle = thread::spawn(move || {
                    for _ in 0..25 {
                        clone.lock().unwrap().check();
                    }
                });
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }
        })
    });
}

criterion_group!(
    benches,
    benchmark_token_bucket,
    benchmark_token_bucket_concurrent_20,
    benchmark_fixed_window,
    benchmark_fixed_window_concurrent_20,
    benchmark_leaking_bucket,
    benchmark_leaking_bucket_concurrent_20,
    benchmark_sliding_window,
    benchmark_sliding_window_concurrent_20,
    benchmark_sliding_window_counter,
    benchmark_sliding_window_counter_concurrent_20,
);
criterion_main!(benches);