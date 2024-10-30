use std::future::Future;
use futures_util::future::BoxFuture;

pub trait NextImpl<I, O, E>: Send + Sync where
    I: Send + Sync,
    O: Send + Sync,
    E: Send + Sync {
    fn call(&self, i: I) -> BoxFuture<'static, Result<O, E>>;
}

impl<F, Fut, I, O, E> NextImpl<I, O, E> for F where
    I: Send + Sync,
    O: Send + Sync,
    E: Send + Sync,
    F: Fn(I) -> Fut + Sync + Send,
    Fut: Future<Output = Result<O, E>> + Send + 'static {
    fn call(&self, i: I) -> BoxFuture<'static, Result<O, E>> {
        Box::pin(self(i))
    }
}

pub struct Next<I, O, E> where
    I: Send + Sync,
    O: Send + Sync,
    E: Send + Sync {
    imp: Box<dyn NextImpl<I, O, E>>
}

impl<I, O, E> Next<I, O, E>
where
    I: Send + Sync,
    O: Send + Sync,
    E: Send + Sync {
    pub fn new<T>(n: T) -> Self where T: NextImpl<I, O, E> + 'static {
        Self {
            imp: Box::new(n)
        }
    }
    pub async fn call(&self, i: I) -> Result<O, E> {
        self.imp.call(i).await
    }
}
