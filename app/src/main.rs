mod server;
mod db;
mod internal;

use server::run;

#[derive(Clone)]
struct App {
    pub data: db::Data,
}

#[tokio::main]
async fn main() {
    let app = App{
        data: db::Data {

        }
    };
    run(app).await;
}
