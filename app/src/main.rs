mod server;
mod db;
mod internal;

use std::sync::Arc;

use server::run;

#[derive(Clone)]
struct App {
    pub data: Arc<dyn db::Data + Sync + Send>,
}

#[tokio::main]
async fn main() {
    let app = App{
        data: Arc::new(db::Postgres {
        }),
    };
    run(app).await;
}
