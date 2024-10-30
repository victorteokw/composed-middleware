use crate::node::Node;

// pub mod next;
// pub mod middleware;
// pub mod layered;
//
// use std::convert::Infallible;
// use crate::layered::Layered;
// use crate::middleware::Middleware;
// use crate::next::{Next, NextImpl};
pub mod node;


#[tokio::main]
async fn main() {
    let mut root = Box::new(Node::new(5));
    root.add_child(Node::new(10));
    root.add_child(Node::new(15));
    let first = root.child(0);
    println!("see first parent: {:?}", first.parent());

    // let middleware_outer = Middleware::<i32, i32, Infallible>::new(|req, next| async move {
    //     println!("outer in");
    //     let result = next.call(req).await?;
    //     println!("outer out");
    //     return Ok(result)
    // });
    // let middleware_inner = Middleware::<i32, i32, Infallible>::new(|req, next| async move {
    //     println!("inner in");
    //     let result = next.call(req).await?;
    //     println!("inner out");
    //     return Ok(result)
    // });
    // let service = Next::<i32, i32, Infallible>::new(|req| async move {
    //     Ok(req + 5)
    // });
    // let layer = Layered::new(
    //     middleware_outer,
    //     Next::new(Layered::new(middleware_inner, service))
    // );
    // let result = layer.call(20).await.unwrap();
    // println!("Hello, world! {}", result);
}
