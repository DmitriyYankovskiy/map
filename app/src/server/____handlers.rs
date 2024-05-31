use axum::{
    extract::State,
    response::IntoResponse,
};

use super::App;

pub async fn index(State(app): State<App>) -> impl IntoResponse {
    "hihi haha 52".to_string()
}

pub async fn update(State(app): State<App>) -> impl IntoResponse {
    "hihi haha 52".to_string()
}