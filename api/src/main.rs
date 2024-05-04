mod routes;
use env_logger;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::new().default_filter_or("info")).init();

    let routes = routes::routes();

    println!("\nServer started at http://localhost:8000");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
