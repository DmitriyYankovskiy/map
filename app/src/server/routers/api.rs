use super::{IntoResponse, Router, get, post, App, State, Path, Project, Machine};
use axum::routing::delete;
use uuid::Uuid;

pub fn index(app: App) -> Router {
    Router::new()
        .nest("/projects", projects(app.clone()))
        //.route("/machines", )
}

fn projects(app: App) -> Router {
    async fn create_project(State(app): State<App>) -> impl IntoResponse {
        serde_json::to_string(&app.data.create_project(Project {
           name: "aboba".to_string(),
           port: 125u16,
           path: "/52".to_string(),
           descr: "525252525252525".to_string(),
        })).unwrap();

        "I create new project".to_string()
    }
    async fn get_project_by_id(State(app): State<App>, Path(id): Path<String>) -> impl IntoResponse {
        serde_json::to_string(&app.data.get_project_by_id(Uuid::parse_str(id.as_str()).unwrap())).unwrap()
    }
    async fn upd_project_by_id(State(app): State<App>, Path(id): Path<String>) -> impl IntoResponse {
        todo!()
    }
    async fn del_project_by_id(State(app): State<App>, Path(id): Path<String>) -> impl IntoResponse {
        todo!()
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

