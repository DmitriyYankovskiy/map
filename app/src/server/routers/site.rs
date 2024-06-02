use super::{IntoResponse, Router, get, AppState, State, Path, Project, Machine};

pub fn index(state: AppState) -> Router {
    Router::new()
        .nest("/projects",  projects(state.clone()))
}

fn projects(state: AppState) -> Router {
    async fn get_list(State(state): State<AppState>) -> impl IntoResponse {
    
    }

    async fn get_by_id(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    
    }

    Router::new()
        .route("/", get(get_list))
        .nest("/:id", Router::new()
            .route("/", get(get_by_id))
            .with_state(state.clone())
        )
        .with_state(state.clone())
}
