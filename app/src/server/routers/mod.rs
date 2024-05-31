pub mod api;
pub mod site;

use axum::{Router, routing::get, response::IntoResponse, extract::{State, Path}};
use super::App;

use crate::db::{project::Project, machine::Machine};