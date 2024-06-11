pub mod api;
pub mod site;

use axum::{Router, routing::{get, post, delete}, response::IntoResponse, extract::{State, Path, Json}};
use super::State as AppState;

use crate::db::models::{Unit, Machine};