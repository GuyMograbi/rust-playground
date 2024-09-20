mod handlers;
mod middleware;
mod routes;
use axum::Router;
use env_logger;

#[tokio::main]
async fn main() {
    env_logger::init();
    let app = Router::new()
        .merge(routes::routes());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
