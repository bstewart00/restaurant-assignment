use crate::{models::{menu::MenuItemId, orders::TableId}, state::SharedAppState};
use axum::{extract::{Path, State}, http::Response, response::IntoResponse, routing::{delete, get, post, put}, Router};

pub fn create_routes() -> Router<SharedAppState> {
    return Router::<SharedAppState>::new()
        .route("/v0/orders/:table_id", post(create_order_handler))
        .route("/v0/orders/:table_id", get(read_order_handler))
        .route("/v0/orders/:table_id", put(update_order_handler))
        .route("/v0/orders/:table_id", delete(delete_order_handler))
        .route("/v0/orders/:table_id/items/:item_id", get(read_order_item_handler))
        .route("/v0/orders/:table_id/items/:item_id", delete(delete_order_item_handler))
}

#[axum::debug_handler]
async fn create_order_handler(
    State(state): State<SharedAppState>,
    Path(table_id): Path<TableId>
) -> Response<axum::body::Body> {
    let db = &state.read().unwrap().db;
    return format!("v0 create_order_handler: {} {}", db.get("foo").unwrap(), table_id).into_response();
}

#[axum::debug_handler]
async fn read_order_handler(
    State(state): State<SharedAppState>,
    Path(table_id): Path<TableId>
) -> Response<axum::body::Body> {
    let db = &state.read().unwrap().db;
    return format!("v0 read_order_handler: {} {}", db.get("foo").unwrap(), table_id).into_response();
}

#[axum::debug_handler]
async fn update_order_handler(
    State(state): State<SharedAppState>,
    Path(table_id): Path<TableId>
) -> Response<axum::body::Body> {
    let db = &state.read().unwrap().db;
    return format!("v0 update_order_handler: {} {}", db.get("foo").unwrap(), table_id).into_response();
}

#[axum::debug_handler]
async fn delete_order_handler(
    State(state): State<SharedAppState>,
    Path(table_id): Path<TableId>
) -> Response<axum::body::Body> {
    let db = &state.read().unwrap().db;
    return format!("v0 delete_order_handler: {} {}", db.get("foo").unwrap(), table_id).into_response();
}

#[axum::debug_handler]
async fn read_order_item_handler(
    State(state): State<SharedAppState>, 
    Path((table_id, item_id)): Path<(TableId, MenuItemId)>
) -> Response<axum::body::Body> {
    let db = &state.read().unwrap().db;
    return format!("v0 read_order_item_handler: {} {} {}", db.get("foo").unwrap(), table_id, item_id).into_response();
}

#[axum::debug_handler]
async fn delete_order_item_handler(
    State(state): State<SharedAppState>, 
    Path((table_id, item_id)): Path<(TableId, MenuItemId)>
) -> Response<axum::body::Body> {
    let db = &state.read().unwrap().db;
    return format!("v0 delete_order_item_handler: {} {} {}", db.get("foo").unwrap(), table_id, item_id).into_response();
}
