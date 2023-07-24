use std::future::Future;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::{sync, time};
use tokio::sync::mpsc;

use crate::error;

pub struct ActionRateLimiter<T> {
    component: T,
}

impl<T> ActionRateLimiter<T> {
    pub async fn invoke<'a, R, F: Future<Output=error::Result<R>> + 'a>(&'a self, f: impl FnOnce(&'a T) -> F) -> error::Result<R> {
        f(&self.component).await
    }
}

pub struct AsyncLeakyBucket<'a> {
    bucket: LeakyBucket<'a>,
    request_take: mpsc::Sender<(f64, Arc<sync::Notify>)>,
    recv_take: mpsc::Receiver<(f64, Arc<sync::Notify>)>,
}

impl<'a> AsyncLeakyBucket<'a> {
    pub fn new(bucket: LeakyBucket) -> Self {
        let (tx, rx) = mpsc::channel::<(f64, Arc<sync::Notify>)>(0);
        let mut async_bucket = AsyncLeakyBucket {
            bucket,
            request_take: tx,
            recv_take: rx,
        };
        tokio::spawn(async {
            async_bucket.task_loop().await
        });
        async_bucket
    }

    async fn task_loop(&mut self) {
        let mut tasks = Vec::<(f64, Arc<sync::Notify>)>::new();

        loop {
            let sleep_time = if let Some(task) = tasks.get(0) {
                task.0
            } else {
                f64::MAX
            };

            let next_wake = time::sleep(Duration::from_secs_f64(sleep_time));

            tokio::select! {
                _ = next_wake => {
                    if let Some(task) = tasks.pop() {
                        task.1.notify_waiters();
                    }
                }
                task = self.recv_take.recv() => {
                    if let Some(task) = task {
                        tasks.push(task);
                    }
                }
            }
        }
    }

    pub async fn take(&mut self, amount: f64) {
        let notify = Arc::new(sync::Notify::new());
        self.request_take.send((amount, notify.clone()));
        notify.notified().await;
    }
}

pub struct LeakyBucket<'a> {
    filled: f64,
    max_bucket_size: f64,
    bucket_fill_rate: f64,
    bucket_fill_interval: Duration,
    last_filled: SystemTime,

    now_fn: Box<dyn Fn() -> SystemTime + 'a>,
}

impl<'a> LeakyBucket<'a> {
    pub fn new(initial_amount: f64, max_bucket_size: f64, bucket_fill_rate: f64, bucket_fill_interval: Duration) -> Self {
        LeakyBucket::new_with_clock(initial_amount, max_bucket_size, bucket_fill_rate, bucket_fill_interval, Box::new(LeakyBucket::now))
    }

    #[cfg(test)]
    pub fn new_with_clock(initial_amount: f64, max_bucket_size: f64, bucket_fill_rate: f64, bucket_fill_interval: Duration, now_fn: Box<dyn Fn() -> SystemTime + 'a>,
    ) -> LeakyBucket<'a> {
        let initial_amount = f64::min(initial_amount, max_bucket_size);
        LeakyBucket {
            filled: initial_amount,
            max_bucket_size,
            bucket_fill_rate,
            bucket_fill_interval,
            last_filled: now_fn(),
            now_fn,
        }
    }

    fn now() -> SystemTime {
        SystemTime::now()
    }

    fn fill_bucket(&mut self) {
        let now = (self.now_fn)();
        match now.duration_since(self.last_filled) {
            Ok(elapsed) => {
                let fill_amount = self.bucket_fill_rate * (elapsed.as_secs_f64() / self.bucket_fill_interval.as_secs_f64());
                self.filled = f64::min(self.filled + fill_amount, self.max_bucket_size);
                self.last_filled = now;
            }
            Err(_) => {}
        }
    }

    pub fn take(&mut self, amount: f64) -> bool {
        self.fill_bucket();

        if amount <= self.filled {
            self.filled -= amount;
            true
        } else {
            false
        }
    }

    pub fn when(&mut self, amount: f64) -> Duration {
        self.fill_bucket();

        if amount <= self.filled {
            Duration::from_secs(0)
        } else {
            let diff = amount - self.filled;
            Duration::from_secs_f64(diff / self.bucket_fill_rate * self.bucket_fill_interval.as_secs_f64())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::ops::{AddAssign};
    use std::time::{Duration, SystemTime};
    use crate::error;
    use crate::rate_limiter::{ActionRateLimiter, AsyncLeakyBucket, LeakyBucket};

    #[test]
    fn test_leaky_bucket() {
        let time = RefCell::new(SystemTime::now());
        let mut b = LeakyBucket::new_with_clock(
            0.,
            100.,
            5.,
            Duration::from_secs(1),
            Box::new(|| time.borrow().clone()),
        );

        assert!(!b.take(5.));

        time.borrow_mut().add_assign(Duration::from_secs(1));
        assert!(b.take(5.));
        assert!(!b.take(5.));
        assert_eq!(Duration::from_secs(1), b.when(5.));
        assert_eq!(Duration::from_secs(2), b.when(10.));

        time.borrow_mut().add_assign(Duration::from_secs(2));
        assert!(b.take(5.));
        assert!(b.take(5.));
        assert!(!b.take(5.));
    }

    #[tokio::test]
    async fn test_async_leaky_bucket() {
        let time = RefCell::new(SystemTime::now());
        let bucket = LeakyBucket::new_with_clock(
            0.,
            100.,
            5.,
            Duration::from_secs(1),
            Box::new(|| time.borrow().clone()),
        );
        let mut async_bucket = AsyncLeakyBucket::new(bucket);

        // assert!(!bucket.take(5.));

        // time.borrow_mut().add_assign(Duration::from_secs(1));
        // assert!(bucket.take(5.));
        // assert!(!bucket.take(5.));
        // assert_eq!(Duration::from_secs(1), bucket.when(5.));
        // assert_eq!(Duration::from_secs(2), bucket.when(10.));
        //
        // time.borrow_mut().add_assign(Duration::from_secs(2));
        // assert!(bucket.take(5.));
        // assert!(bucket.take(5.));
        // assert!(!bucket.take(5.));
    }

    #[tokio::test]
    async fn test_invoke_evaluates() -> Result<(), error::Error> {
        let a = ActionRateLimiter::<i64> {
            component: 1
        };

        let result = a.invoke(|x: &i64| async {
            Ok(*x + 1)
        }).await?;
        assert_eq!(2, result);

        let result = a.invoke(|x: &i64| async {
            Ok(*x + 2)
        }).await?;
        assert_eq!(3, result);
        Ok(())
    }
}
