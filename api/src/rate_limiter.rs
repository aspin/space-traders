use std::future::Future;
use crate::error;

pub struct ActionRateLimiter<T> {
    component: T,
}

impl<T> ActionRateLimiter<T> {
    pub async fn invoke<'a, R, F: Future<Output=error::Result<R>> + 'a>(&'a self, f: impl FnOnce(&'a T) -> F) -> error::Result<R> {
        f(&self.component).await
    }
}

#[cfg(test)]
mod tests {
    use crate::error;
    use crate::rate_limiter::ActionRateLimiter;

    #[tokio::test]
    async fn test_invoke_evaluates() -> Result<(), error::Error> {
        let a = ActionRateLimiter::<i64> {
            component: 1
        };

        let result = a.invoke(|x: &i64| async {
            Ok(*x + 1)
        }).await?;
        assert_eq!(2, result);
        Ok(())
    }
}
