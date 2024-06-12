use axum::Router;

use super::{AppState, State};

pub fn index(state: AppState) -> Router {
    Router::new()
        .route("/update", todo!())
}