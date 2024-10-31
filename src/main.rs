pub mod next;
pub mod middleware;
pub mod layered;

use std::convert::Infallible;
use crate::layered::Layered;
use crate::middleware::Middleware;
use crate::next::{Next, NextImpl};

async fn outer_fn<'a>(req: i32, next: &'a Next<'a, i32, i32, Infallible>) -> Result<i32, Infallible> {
    next.call(req + 50).await
}

async fn inner_fn<'a>(req: i32, next: &'a Next<'a, i32, i32, Infallible>) -> Result<i32, Infallible> {
    next.call(req + 30).await
}

#[tokio::main]
async fn main() {

    let middleware_outer = Middleware::<i32, i32, Infallible>::new(outer_fn);
    let middleware_inner = Middleware::<i32, i32, Infallible>::new(inner_fn);
    // let middleware_outer = Middleware::<i32, i32, Infallible>::new(|req, next: &Next<i32, i32, Infallible>| async move {
    //     println!("outer in");
    //     let result = next.call(req).await?;
    //     println!("outer out");
    //     return Ok(result)
    // });
    // let middleware_inner = Middleware::<i32, i32, Infallible>::new(|req, next: &Next<i32, i32, Infallible>| async move {
    //     println!("inner in");
    //     let result = next.call(req).await?;
    //     println!("inner out");
    //     return Ok(result)
    // });
    let service = Next::<i32, i32, Infallible>::new(|req| async move {
        Ok(req + 5)
    });
    let layer = Layered::new(
        middleware_outer,
        Next::new(Layered::new(middleware_inner, service))
    );
    let result = layer.call(20).await.unwrap();
    println!("Hello, world! {}", result);
}
