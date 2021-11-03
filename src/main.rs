use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    let PORT = match env::var("PORT") {
        Ok(port) => port,
        Err(e) => eprintln!("{:?}", e),
    };
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    warp::serve(hello).run(([127, 0, 0, 1], PORT)).await;
}
