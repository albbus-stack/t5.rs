#[macro_use]
extern crate dotenvy_macro;

mod auth;
mod db;
mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::new().default_filter_or("info")).init();

    let routes = routes::routes();

    println!("\nAPP_URL: {}", dotenv!("APP_URL"));
    println!("\n\x1b[1;32mServer started at http://localhost:8000\n\x1b[0m");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
