mod server;
mod db;
mod internal;

use server::run;

#[derive(Clone)]
struct App {

}

#[tokio::main]
async fn main() {
    let app = App{};
    run(app).await;
}
