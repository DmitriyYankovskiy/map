use axum::{response::IntoResponse, Router};
use uuid::Uuid;
use super::State as AppState;
use axum::{extract::{State, Json, Path}, routing::{get, post}};
use crate::pinger::Mark;

pub fn index(state: AppState) -> Router {
    async fn track(State(state): State<AppState>, Json(new_marks): Json<Vec<(u128, Mark)>>) -> impl IntoResponse {
        state.pinger.track(new_marks.into_iter().map(|(id, mark)| (Uuid::from_u128(id), mark)).collect()).await;
    }

    async fn get_status(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
        let id = if let Ok(id) = Uuid::parse_str(id.as_str()) {
            id
        } else {
            return "incorrect id";
        };
        if state.pinger.get_status(&id).await {
            "online"
        } else {
            "ofline"
        }
    }

    async fn get_list(State(state): State<AppState>) -> impl IntoResponse {
        serde_json::to_string(&state.pinger.get_list().await.into_iter().map(|id| id.to_string()).collect::<Vec<String>>()).unwrap()
    }
        


    Router::new()
        .route("/track", post(track))
        .route("/online_list", get(get_list))
        .route("/status/:id", get(get_status))
        .with_state(state)
}

