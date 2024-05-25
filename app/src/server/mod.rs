mod handlers;

use axum::{handler, routing::get, Router};
use std::net::SocketAddr;

use crate::App;



pub async fn run(app: App) {
    let router = Router::new()
        .route("/", get(handlers::index))
        .with_state(app);
    let addr = SocketAddr::from(([127, 0, 0, 1], 5252));
    let listner = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listner, router).await.unwrap();
}