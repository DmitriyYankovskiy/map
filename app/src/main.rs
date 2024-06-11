mod server;
mod db;
mod services;

use std::sync::Arc;

use services::pinger::Pinger;

use {
    server::run,
    services::pinger,
};

type Ip = [u8; 4];
type Port = u16;

#[derive(Clone)]
struct State {
    pub data: Arc<dyn db::Data + Sync + Send>,
    pub pinger: Arc<pinger::Pinger>,
}

#[tokio::main]
async fn main() {
    let state = State {
        data: Arc::new(db::postgres::connect()),
        pinger: Arc::new(pinger::Pinger::new()),
    };
    run(state).await;
}
