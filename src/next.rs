use std::future::Future;
use futures_util::future::BoxFuture;

pub trait NextImpl<'a, I, O, E>: Send + Sync where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a {
    fn call(&'a self, i: I) -> BoxFuture<'a, Result<O, E>>;
}

impl<'a, F, Fut, I, O, E> NextImpl<'a, I, O, E> for F where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a,
    F: Fn(I) -> Fut + Sync + Send + 'static,
    Fut: Future<Output = Result<O, E>> + Send + 'a {
    fn call(&'a self, i: I) -> BoxFuture<'a, Result<O, E>> {
        Box::pin(self(i))
    }
}

pub struct Next<'a, I, O, E> where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a {
    imp: Box<dyn NextImpl<'a, I, O, E>>
}

impl<'a, I, O, E> Next<'a, I, O, E>
where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a {
    pub fn new<T>(n: T) -> Self where T: NextImpl<'a, I, O, E> + 'static {
        Self {
            imp: Box::new(n)
        }
    }
    pub async fn call(&'a self, i: I) -> Result<O, E> {
        self.imp.call(i).await
    }
}
