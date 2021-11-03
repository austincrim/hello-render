use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    let is_prod = match env::var("RENDER") {
        Ok(_) => true,
        Err(_) => false,
    };

    let host = if is_prod {
        ([0, 0, 0, 0], 10000)
    } else {
        ([127, 0, 0, 1], 3000)
    };

    let hello = warp::path::end().map(|| format!("Hello, world!"));
    let hello_name = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    let routes = warp::any().and(hello).or(hello_name);

    println!("serving on port {}", host.1);
    warp::serve(routes).run(host).await;
}
