mod handlers;
mod routes;
use env_logger;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::new().default_filter_or("info")).init();

    let routes = routes::routes();

    println!("\n\x1b[1;32mServer started at http://localhost:8000\n\x1b[0m");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}