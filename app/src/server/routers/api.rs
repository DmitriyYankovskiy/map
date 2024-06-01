use super::{IntoResponse, Router, get, post, App, State, Path, Project, Machine, Json};
use axum::routing::delete;
use uuid::Uuid;

pub fn index(app: App) -> Router {
    Router::new()
        .nest("/projects", projects(app.clone()))
        //.route("/machines", )
}

fn projects(app: App) -> Router {
    async fn create_project(State(app): State<App>, Json(project): Json<Project>) -> impl IntoResponse {
        app.data.create_project(&project);
        serde_json::to_string(&project).unwrap()
    }
    async fn get_project_by_id(State(app): State<App>, Path(id): Path<String>) -> impl IntoResponse {
        serde_json::to_string(&app.data.get_project_by_id(Uuid::parse_str(id.as_str()).unwrap())).unwrap()
    }
    async fn upd_project_by_id(State(app): State<App>, Path(id): Path<String>, Json(project): Json<Project>) -> impl IntoResponse {
        app.data.update_project(Uuid::parse_str(id.as_str()).unwrap(), &project);
        serde_json::to_string(&project).unwrap();
    }
    async fn del_project_by_id(State(app): State<App>, Path(id): Path<String>) -> impl IntoResponse {
        app.data.delete_project(Uuid::parse_str(id.as_str()).unwrap());
        "I delete project".to_string()
    }

    Router::new()
        .route("/", post(create_project))
        .nest("/:id", Router::new()
            .route("/", get(get_project_by_id))
            .route("/", post(upd_project_by_id))
            .route("/", delete(del_project_by_id))
            .with_state(app.clone())
        )
        .with_state(app)
}

