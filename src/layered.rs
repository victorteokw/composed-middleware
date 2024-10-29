use futures_util::future::BoxFuture;
use crate::middleware::Middleware;
use crate::next::{Next, NextImpl};

pub struct Layered<I, O, E> where
    I: Send + Sync + 'static,
    O: Send + Sync + 'static,
    E: Send + Sync + 'static {
    middleware: Middleware<I, O, E>,
    next: Next<I, O, E>,
}

impl<I, O, E> Layered<I, O, E> where
    I: Send + Sync + 'static,
    O: Send + Sync + 'static,
    E: Send + Sync + 'static {
    pub fn new(middleware: Middleware<I, O, E>, next: Next<I, O, E>) -> Self {
        Self { middleware, next }
    }

    pub async fn call(self, i: I) -> Result<O, E> {
        self.middleware.call(i, self.next).await
    }
}

impl<I, O, E> NextImpl<I, O, E> for Layered<I, O, E> where
    I: Send + Sync + 'static,
    O: Send + Sync + 'static,
    E: Send + Sync + 'static {
    fn call(self, i: I) -> BoxFuture<'static, Result<O, E>> {
        Box::pin(self.call(i))
    }
}
