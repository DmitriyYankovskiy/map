mod pinger;
mod server;

use std::sync::Arc;

#[derive(Clone)]
struct State {
    pub pinger: Arc<pinger::Pinger>,
}

#[tokio::main]
async fn main() {
    let state = State {
        pinger: Arc::new(pinger::Pinger::new())
    };
    server::run(state).await
}
