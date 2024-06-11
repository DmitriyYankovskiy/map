use super::{IntoResponse, Router, get, post, delete, AppState, State, Path, Unit, Machine, Json};
use uuid::Uuid;

pub fn index(state: AppState) -> Router {
    Router::new()
        .nest("/units", units(state.clone()))
        .nest("/machines", machines(state.clone()))
}

fn machines(state: AppState) -> Router {
    async fn create(State(state): State<AppState>, Json(machine): Json<Machine>) -> impl IntoResponse {
        state.data.create_machine(&machine);
        serde_json::to_string(&machine).unwrap()
    }
    async fn get_by_id(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
        let id = match Uuid::parse_str(id.as_str()) {
            Ok(id) => id,
            Err(text) => return text.to_string(),
        };
        match state.data.get_machine_by_id(id) {
            Ok(machine) => serde_json::to_string(&machine).unwrap(),
            Err(text) => text,
        }
    }
    async fn upd_by_id(State(state): State<AppState>, Path(id): Path<String>, Json(machine): Json<Machine>) -> impl IntoResponse {
        let id = match Uuid::parse_str(id.as_str()) {
            Ok(id) => id,
            Err(text) => return text.to_string(),
        };
        if let Err(text) = state.data.upd_machine_by_id(id, &machine) {
            return text
        }
        serde_json::to_string(&machine).unwrap();
        "I update unit".to_string()
    }
    async fn del_by_id(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
        let id = match Uuid::parse_str(id.as_str()) {
            Ok(id) => id,
            Err(text) => return text.to_string(),
        };
        if let Err(text) = state.data.del_machine_by_id(id) {
            return text
        };
        "I delete unit".to_string()
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




fn units(state: AppState) -> Router {
    async fn create(State(state): State<AppState>, Json(unit): Json<Unit>) -> impl IntoResponse {
        state.data.create_unit(&unit);
        serde_json::to_string(&unit).unwrap()
    }
    async fn get_by_id(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
        let id = match Uuid::parse_str(id.as_str()) {
            Ok(id) => id,
            Err(text) => return text.to_string(),
        };
        match state.data.get_unit_by_id(id) {
            Ok(unit) => serde_json::to_string(&unit).unwrap(),
            Err(text) => text,
        }
    }
    async fn upd_by_id(State(state): State<AppState>, Path(id): Path<String>, Json(unit): Json<Unit>) -> impl IntoResponse {
        let id = match Uuid::parse_str(id.as_str()) {
            Ok(id) => id,
            Err(text) => return text.to_string(),
        };
        if let Err(text) = state.data.upd_unit_by_id(id, &unit) {
            return text
        }
        serde_json::to_string(&unit).unwrap();
        "I update unit".to_string()
    }
    async fn del_by_id(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
        let id = match Uuid::parse_str(id.as_str()) {
            Ok(id) => id,
            Err(text) => return text.to_string(),
        };
        if let Err(text) = state.data.del_unit_by_id(id) {
            return text
        };
        "I delete unit".to_string()
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

