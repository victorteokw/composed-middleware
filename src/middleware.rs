use std::future::Future;
use std::sync::Arc;
use futures_util::future::BoxFuture;
use crate::next::Next;

pub trait MiddlewareImpl<I, O, E>: Send + Sync where
    I: Send + Sync,
    O: Send + Sync,
    E: Send + Sync {
    fn call(&self, i: I, next: &Next<I, O, E>) -> BoxFuture<Result<O, E>>;
}

impl<F, Fut, I, O, E> MiddlewareImpl<I, O, E> for F where
    I: Send + Sync,
    O: Send + Sync,
    E: Send + Sync,
    for <'a> F: Fn(I, &'a Next<I, O, E>) -> Fut + Sync + Send,
    Fut: Future<Output = Result<O, E>> + Send + 'static {
    fn call(&self, i: I, next: &Next<I, O, E>) -> BoxFuture<'static, Result<O, E>> {
        Box::pin(self(i, next))
    }
}

pub struct Middleware<I, O, E> where
    I: Send + Sync,
    O: Send + Sync,
    E: Send + Sync {
    imp: Arc<dyn MiddlewareImpl<I, O, E>>
}

impl <I, O, E> Middleware<I, O, E> where
    I: Send + Sync,
    O: Send + Sync,
    E: Send + Sync {
    pub fn new<F, Fut>(f: F) -> Self where
        I: Send + Sync,
        O: Send + Sync,
        E: Send + Sync,
        F: Fn(I, &Next<I, O, E>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<O, E>> + Send + 'static {
        Self {
            imp: Arc::new(f)
        }
    }

    pub async fn call(&self, i: I, next: &Next<I, O, E>) -> Result<O, E> {
        self.imp.call(i, next).await
    }
}

impl<I, O, E> Clone for Middleware<I, O, E> where
    I: Send + Sync,
    O: Send + Sync,
    E: Send + Sync {
    fn clone(&self) -> Self {
        Self {
            imp: self.imp.clone()
        }
    }
}