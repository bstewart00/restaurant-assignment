use crate::{
    models::orders::TableOrderItem,
    persistence::persistence::{CreateOrderError, Persistence, ReadOrderError, ReadOrderItemError},
    state::SharedAppState,
};
use axum::{
    extract::{Path, State},
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};

use super::{
    client_params::{from_client_item, from_client_item_id, from_client_table_id, CreateOrUpdateOrderParams},
    view_models::{to_order_item_detail_view_model, to_order_view_model},
};

pub fn create_routes() -> Router<SharedAppState> {
    return Router::<SharedAppState>::new()
        .route("/v0/orders/:table_id", post(create_order_handler))
        .route("/v0/orders/:table_id", get(read_order_handler))
        .route("/v0/orders/:table_id", put(update_order_handler))
        .route("/v0/orders/:table_id", delete(delete_order_handler))
        .route("/v0/orders/:table_id/items/:item_id", get(read_order_item_handler))
        .route("/v0/orders/:table_id/items/:item_id", delete(delete_order_item_handler));
}

#[axum::debug_handler]
async fn create_order_handler(State(state): State<SharedAppState>, Path(client_table_id): Path<String>, Json(payload): Json<CreateOrUpdateOrderParams>) -> Response<axum::body::Body> {
    let app_state = &mut state.write().await;
    let persistence = &mut app_state.persistence;
    let table_id = from_client_table_id(&client_table_id);

    let new_items = payload.items.iter().map(|i| from_client_item(i)).collect::<Vec<TableOrderItem>>();

    let order = persistence.create_order(&table_id, &new_items).await;
    return order.map_or_else(|err| create_error_response(err), |o| (StatusCode::CREATED, axum::Json(to_order_view_model(o))).into_response());
}

async fn read_order_handler(State(state): State<SharedAppState>, Path(client_table_id): Path<String>) -> Response<axum::body::Body> {
    let app_state = &state.read().await;
    let persistence = &app_state.persistence;
    let table_id = from_client_table_id(&client_table_id);
    let order = persistence.find_order(&table_id).await;

    return order.map_or_else(|err| create_error_response(err), |o| (StatusCode::OK, axum::Json(to_order_view_model(o))).into_response());
}

async fn update_order_handler(State(state): State<SharedAppState>, Path(client_table_id): Path<String>, Json(payload): Json<CreateOrUpdateOrderParams>) -> Response<axum::body::Body> {
    let app_state = &mut state.write().await;
    let persistence = &mut app_state.persistence;
    let table_id = from_client_table_id(&client_table_id);

    let new_items = payload.items.iter().map(|i| from_client_item(i)).collect::<Vec<TableOrderItem>>();

    let order = persistence.update_order(&table_id, &new_items).await;
    return order.map_or_else(|err| create_error_response(err), |o| (StatusCode::OK, axum::Json(to_order_view_model(o))).into_response());
}

async fn delete_order_handler(State(state): State<SharedAppState>, Path(client_table_id): Path<String>) -> Response<axum::body::Body> {
    let app_state = &mut state.write().await;
    let persistence = &mut app_state.persistence;
    let table_id = from_client_table_id(&client_table_id);

    let result = persistence.delete_order(&table_id).await;
    return result.map_or_else(|err| create_error_response(err), |_| (StatusCode::NO_CONTENT, ()).into_response());
}

async fn read_order_item_handler(State(state): State<SharedAppState>, Path((client_table_id, client_item_id)): Path<(String, String)>) -> Response<axum::body::Body> {
    let app_state = &state.read().await;
    let persistence = &app_state.persistence;
    let table_id = from_client_table_id(&client_table_id);
    let item_id = from_client_item_id(&client_item_id);
    let order = persistence.find_order(&table_id).await;

    return order
        .map_err(|err| create_error_response(err))
        .and_then(|o| {
            o.items
                .get(&item_id)
                .ok_or_else(|| create_error_response(ReadOrderItemError::OrderItemNotFound(item_id.to_string())))
        })
        .map_or_else(|err_response| err_response, |i| (StatusCode::OK, axum::Json(to_order_item_detail_view_model(i))).into_response());
}

async fn delete_order_item_handler(State(state): State<SharedAppState>, Path((client_table_id, client_item_id)): Path<(String, String)>) -> Response<axum::body::Body> {
    let app_state = &mut state.write().await;
    let persistence = &mut app_state.persistence;
    let table_id = from_client_table_id(&client_table_id);
    let item_id = from_client_item_id(&client_item_id);
    let order = persistence.delete_order_item(&table_id, &item_id).await;

    return order.map_or_else(|err| create_error_response(err), |o| (StatusCode::OK, axum::Json(to_order_view_model(o))).into_response());
}

fn create_error_response<E>(err: E) -> Response<axum::body::Body>
where
    E: Clone + ToString,
    StatusCode: From<E>,
{
    return (StatusCode::from(err.clone()), err.to_string()).into_response();
}

impl From<CreateOrderError> for StatusCode {
    fn from(value: CreateOrderError) -> Self {
        return match value {
            CreateOrderError::OrderAlreadyExistsForTable(_) => Self::CONFLICT,
        };
    }
}

impl From<ReadOrderError> for StatusCode {
    fn from(value: ReadOrderError) -> Self {
        return match value {
            ReadOrderError::OrderNotFound(_) => Self::NOT_FOUND,
        };
    }
}

impl From<ReadOrderItemError> for StatusCode {
    fn from(value: ReadOrderItemError) -> Self {
        return match value {
            ReadOrderItemError::OrderNotFound(_) => Self::NOT_FOUND,
            ReadOrderItemError::OrderItemNotFound(_) => Self::NOT_FOUND,
        };
    }
}
