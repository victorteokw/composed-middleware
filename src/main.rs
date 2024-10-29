use std::future::Future;
use futures_util::future::BoxFuture;

pub trait NextImpl<I, O, E>: Send + Sync where
    I: Send + Sync + 'static,
    O: Send + Sync + 'static,
    E: Send + Sync + 'static {
    fn call(&self, i: I) -> BoxFuture<'static, Result<O, E>>;
    fn call_once(self, i: I) -> BoxFuture<'static, Result<O, E>>;
}

impl<F, Fut, I, O, E> NextImpl<I, O, E> for F where
    I: Send + Sync + 'static,
    O: Send + Sync + 'static,
    E: Send + Sync + 'static,
    F: Fn(I) -> Fut + Sync + Send,
    Fut: Future<Output = Result<O, E>> + Send + 'static {
    fn call(&self, i: I) -> BoxFuture<'static, Result<O, E>> {
        Box::pin(self(i))
    }
    fn call_once(self, i: I) -> BoxFuture<'static, Result<O, E>> {
        Box::pin(self(i))
    }
}

pub struct Next<I, O, E> where
    I: Send + Sync + 'static,
    O: Send + Sync + 'static,
    E: Send + Sync + 'static {
    imp: Box<dyn NextImpl<I, O, E>>
}

impl<I, O, E> Next<I, O, E> where
    I: Send + Sync + 'static,
    O: Send + Sync + 'static,
    E: Send + Sync + 'static {
    pub fn new<F, Fut>(f: F) -> Self where
        I: Send + Sync + 'static,
        O: Send + Sync + 'static,
        E: Send + Sync + 'static,
        F: Fn(I) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<O, E>> + Send + 'static {
        Self {
            imp: Box::new(f)
        }
    }
    pub async fn call(&self, i: I) -> Result<O, E> {
        self.imp.call(i).await
    }
}

pub trait MiddlewareImpl<I, O, E>: Send + Sync where
    I: Send + Sync + 'static,
    O: Send + Sync + 'static,
    E: Send + Sync + 'static {
    fn call_ref(&self, i: I, next: &Next<I, O, E>) -> BoxFuture<'static, Result<O, E>>;
}

impl<F, Fut, I, O, E> MiddlewareImpl<I, O, E> for F where
    I: Send + Sync + 'static,
    O: Send + Sync + 'static,
    E: Send + Sync + 'static,
    F: Fn(I, &Next<I, O, E>) -> Fut + Sync + Send,
    Fut: Future<Output = Result<O, E>> + Send + 'static {
    fn call_ref(&self, i: I, next: &Next<I, O, E>) -> BoxFuture<'static, Result<O, E>> {
        Box::pin(self(i, next))
    }
}

pub struct QuantaMiddleware<I, O, E> where I: Send + Sync + 'static, O: Send + Sync + 'static, E: Send + Sync + 'static {
    imp: Box<dyn MiddlewareImpl<I, O, E>>
}

impl<I, O, E> QuantaMiddleware<I, O, E> where I: Send + Sync + 'static, O: Send + Sync + 'static, E: Send + Sync + 'static {
    pub fn new<F, Fut>(f: F) -> Self where
        I: Send + Sync + 'static,
        O: Send + Sync + 'static,
        E: Send + Sync + 'static,
        F: Fn(I, &Next<I, O, E>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<O, E>> + Send + 'static {
        Self {
            imp: Box::new(f)
        }
    }
    pub async fn call_ref(&self, i: I, n: &Next<I, O, E>) -> Result<O, E> {
        self.imp.call_ref(i, n).await
    }
}

pub struct CallRef<I, O, E> where
    I: Send + Sync + 'static,
    O: Send + Sync + 'static,
    E: Send + Sync + 'static {
    middleware_ref: &'static Middleware<I, O, E>,
    next_ref: &'static Next<I, O, E>,
}
impl<I, O, E> CallRef<I, O, E> where
    I: Send + Sync + 'static,
    O: Send + Sync + 'static,
    E: Send + Sync + 'static {
    pub fn new(middleware: &Middleware<I, O, E>, next: &Next<I, O, E>) -> Self {
        Self {
            middleware_ref: unsafe { &*(middleware as *const Middleware<I, O, E>) },
            next_ref: unsafe { &*(next as *const Next<I, O, E>) },
        }
    }

    pub fn middleware(&self) -> &'static Middleware<I, O, E> {
        self.middleware_ref
    }

    pub fn next(&self) -> &'static Next<I, O, E> {
        self.next_ref
    }
}
unsafe impl<I, O, E> Send for CallRef<I, O, E> where
    I: Send + Sync + 'static,
    O: Send + Sync + 'static,
    E: Send + Sync + 'static { }
unsafe impl<I, O, E> Sync for CallRef<I, O, E> where
    I: Send + Sync + 'static,
    O: Send + Sync + 'static,
    E: Send + Sync + 'static { }

pub struct LinkedMiddleware<I, O, E> where
    I: Send + Sync + 'static,
    O: Send + Sync + 'static,
    E: Send + Sync + 'static {
    outer: Box<Middleware<I, O, E>>,
    inner: Box<Middleware<I, O, E>>,
}

impl<I, O, E> LinkedMiddleware<I, O, E> where
    I: Send + Sync + 'static,
    O: Send + Sync + 'static,
    E: Send + Sync + 'static {
    pub fn new(outer: Middleware<I, O, E>, inner: Middleware<I, O, E>) -> Self {
        Self {
            outer: Box::new(outer),
            inner: Box::new(inner),
        }
    }
    pub async fn call_ref(&self, i: I, n: &Next<I, O, E>) -> Result<O, E> {
        let refs = CallRef::new(self.inner.as_ref(), n);
        let n_ref = refs.next();
        let inner_ref = refs.middleware();
        let next = Next::new(|i| async {
            inner_ref.call_ref(i, n_ref).await
        });
        self.outer.call(i, next).await
    }
}

pub enum Middleware<I, O, E> where I: Send + Sync + 'static, O: Send + Sync + 'static, E: Send + Sync + 'static {
    LinkedMiddleware(LinkedMiddleware<I, O, E>),
    QuantaMiddleware(QuantaMiddleware<I, O, E>),
}

impl<I, O, E> Middleware<I, O, E> where I: Send + Sync + 'static, O: Send + Sync + 'static, E: Send + Sync + 'static {
    pub async fn call(&self, i: I, n: Next<I, O, E>) -> Result<O, E> {
        match self {
            Middleware::LinkedMiddleware(linked) => linked.call_ref(i, &n).await,
            Middleware::QuantaMiddleware(quanta) => quanta.call_ref(i, &n).await,
        }
    }

    pub async fn call_ref(&self, i: I, n: &Next<I, O, E>) -> Result<O, E> {
        match self {
            Middleware::LinkedMiddleware(linked) => linked.call_ref(i, &n).await,
            Middleware::QuantaMiddleware(quanta) => quanta.call_ref(i, &n).await,
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
