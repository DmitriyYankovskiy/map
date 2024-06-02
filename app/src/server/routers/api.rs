use super::{IntoResponse, Router, get, post, delete, AppState, State, Path, Project, Machine, Json};
use uuid::Uuid;

pub fn index(state: AppState) -> Router {
    Router::new()
        .nest("/projects", projects(state.clone()))
        .nest("/machines", machines(state.clone()))
}

fn machines(state: AppState) -> Router {
    async fn create(State(state): State<AppState>, Json(machine): Json<Machine>) -> impl IntoResponse {
        state.app.create_machine(&machine);
        serde_json::to_string(&machine).unwrap()
    }
    async fn get_by_id(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
        let id = match Uuid::parse_str(id.as_str()) {
            Ok(id) => id,
            Err(text) => return text.to_string(),
        };
        match state.app.get_machine_by_id(id) {
            Ok(machine) => serde_json::to_string(&machine).unwrap(),
            Err(text) => text,
        }
    }
    async fn upd_by_id(State(state): State<AppState>, Path(id): Path<String>, Json(machine): Json<Machine>) -> impl IntoResponse {
        let id = match Uuid::parse_str(id.as_str()) {
            Ok(id) => id,
            Err(text) => return text.to_string(),
        };
        if let Err(text) = state.app.upd_machine_by_id(id, &machine) {
            return text
        }
        serde_json::to_string(&machine).unwrap();
        "I update project".to_string()
    }
    async fn del_by_id(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
        let id = match Uuid::parse_str(id.as_str()) {
            Ok(id) => id,
            Err(text) => return text.to_string(),
        };
        if let Err(text) = state.app.del_machine_by_id(id) {
            return text
        };
        "I delete project".to_string()
    }

    Router::new()
        .route("/", post(create))
        .nest("/:id", Router::new()
            .route("/", get(get_by_id))
            .route("/", post(upd_by_id))
            .route("/", delete(del_by_id))
            .with_state(state.clone())
        )
        .with_state(state)
}

fn projects(state: AppState) -> Router {
    async fn create(State(state): State<AppState>, Json(project): Json<Project>) -> impl IntoResponse {
        state.app.create_project(&project);
        serde_json::to_string(&project).unwrap()
    }
    async fn get_by_id(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
        let id = match Uuid::parse_str(id.as_str()) {
            Ok(id) => id,
            Err(text) => return text.to_string(),
        };
        match state.app.get_project_by_id(id) {
            Ok(project) => serde_json::to_string(&project).unwrap(),
            Err(text) => text,
        }
    }
    async fn upd_by_id(State(state): State<AppState>, Path(id): Path<String>, Json(project): Json<Project>) -> impl IntoResponse {
        let id = match Uuid::parse_str(id.as_str()) {
            Ok(id) => id,
            Err(text) => return text.to_string(),
        };
        if let Err(text) = state.app.upd_project_by_id(id, &project) {
            return text
        }
        serde_json::to_string(&project).unwrap();
        "I update project".to_string()
    }
    async fn del_by_id(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
        let id = match Uuid::parse_str(id.as_str()) {
            Ok(id) => id,
            Err(text) => return text.to_string(),
        };
        if let Err(text) = state.app.del_project_by_id(id) {
            return text
        };
        "I delete project".to_string()
    }

    Router::new()
        .route("/", post(create))
        .nest("/:id", Router::new()
            .route("/", get(get_by_id))
            .route("/", post(upd_by_id))
            .route("/", delete(del_by_id))
            .with_state(state.clone())
        )
        .with_state(state)
}

