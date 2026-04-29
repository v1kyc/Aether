// Imports
use crate::router::router;

mod response;
mod router;
mod tools;

#[tokio::main]
async fn main() {
    let tcp_listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(tcp_listener, router()).await.unwrap();
}
