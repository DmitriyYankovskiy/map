mod routers;

use axum::Router;
use std::net::SocketAddr;

use crate::State;



pub async fn run(state: State) {
    let router = Router::new()
        .nest("/api", routers::api::index(state.clone()));
    let addr = SocketAddr::from(([127, 0, 0, 1], 5252));
    let listner = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listner, router).await.unwrap();
}