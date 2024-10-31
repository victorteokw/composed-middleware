use std::future::Future;
use std::sync::Arc;
use futures_util::future::BoxFuture;
use crate::next::Next;

pub trait MiddlewareImpl<'a, I, O, E>: Send + Sync where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a {
    fn call(&'a self, i: I, next: &'a Next<I, O, E>) -> BoxFuture<'a, Result<O, E>>;
}

impl<'a, F, Fut, I, O, E> MiddlewareImpl<'a, I, O, E> for F where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a,
    F: Fn(I, &Next<I, O, E>) -> Fut + Sync + Send + 'static,
    Fut: Future<Output = Result<O, E>> + Send + 'a {
    fn call(&'a self, i: I, next: &'a Next<I, O, E>) -> BoxFuture<'a, Result<O, E>> {
        Box::pin(self(i, next))
    }
}

pub struct Middleware<'a, I, O, E> where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a {
    imp: Arc<dyn MiddlewareImpl<'a, I, O, E>>
}

impl <'a, I, O, E> Middleware<'a, I, O, E> where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a {
    pub fn new<T>(t: T) -> Self where
        T: MiddlewareImpl<'a, I, O, E> + 'static {
        Self {
            imp: Arc::new(t)
        }
    }

    pub async fn call(&'a self, i: I, next: &'a Next<'a, I, O, E>) -> Result<O, E> {
        self.imp.call(i, next).await
    }
}

impl<'a, I, O, E> Clone for Middleware<'a, I, O, E> where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a {
    fn clone(&self) -> Self {
        Self {
            imp: self.imp.clone()
        }
    }
}