use futures_util::future::BoxFuture;
use crate::middleware::Middleware;
use crate::next::{Next, NextImpl};

pub struct Layered<'a, I, O, E> where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a {
    middleware: Middleware<'a, I, O, E>,
    next: Next<'a, I, O, E>,
}

impl<'a, I, O, E> Layered<'a, I, O, E> where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a {
    pub fn new(middleware: Middleware<'a, I, O, E>, next: Next<'a, I, O, E>) -> Self {
        Self { middleware, next }
    }

    pub async fn call(&'a self, i: I) -> Result<O, E> {
        self.middleware.call(i, &self.next).await
    }
}

impl<'a, I, O, E> NextImpl<'a, I, O, E> for Layered<'a, I, O, E> where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a {
    fn call(&'a self, i: I) -> BoxFuture<'a, Result<O, E>> {
        Box::pin(self.call(i))
    }
}
