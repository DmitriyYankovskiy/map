mod server;
mod db;
mod internal;

use std::sync::Arc;

use {
    server::run,
    internal::App,
};

#[derive(Clone)]
struct State {
    pub app: App,
}

#[tokio::main]
async fn main() {
    let state = State {
        app: App {
            data: Arc::new(db::Postgres {
            }),
        }        
    };
    run(state).await;
}
