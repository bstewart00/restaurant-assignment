use crate::state::SharedAppState;
use axum::{extract::State, http::Response, response::IntoResponse, routing::get, Router};

pub fn create_routes() -> Router<SharedAppState> {
    return Router::<SharedAppState>::new().route("/v0/foo", get(foo_handler));
}

async fn foo_handler(State(state): State<SharedAppState>) -> Response<axum::body::Body> {
    let db = &state.read().unwrap().db;

    return format!("v0 foo: {}", db.get("foo").unwrap()).into_response();
}
