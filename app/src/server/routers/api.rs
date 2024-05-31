use super::{IntoResponse, Router, get, App, State, Path};
use axum::routing::post;
use uuid::Uuid;

pub fn index(app: App) -> Router {
    Router::new()
        .nest("/projects", projects(app.clone()))
        //.route("/machines", )
}

fn projects(app: App) -> Router {
    async fn get_project_by_id(State(app): State<App>, Path(id): Path<String>) -> impl IntoResponse{
        serde_json::to_string(&app.data.get_project_by_id(Uuid::parse_str(id.as_str()).unwrap())).unwrap()
    }

    async fn create_project(State(app): State<App>, Path(id): Path<String>) -> impl IntoResponse{
        serde_json::to_string(&app.data.create_project(Uuid::parse_str(id.as_str()).unwrap())).unwrap()
    }

    Router::new()
        .nest("/:id", Router::new()
            .route("/", get(get_project_by_id))
            .route("/", post(create_project))
            .with_state(app)
        )
}

