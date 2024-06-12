mod pinger;
mod server;

use std::sync::Arc;

#[derive(Clone)]
struct State {
    pub pinger: Arc<pinger::Pinger>,
}

fn main() {
    
}
