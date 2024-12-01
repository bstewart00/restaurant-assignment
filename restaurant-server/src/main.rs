use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::Router;
use state::{AppState, SharedAppState};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod models;
mod persistence;
mod state;
mod table_orders_service;

#[cfg(test)]
mod tests {
    mod memory_persistence_tests;
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let mut app_state = AppState { db: HashMap::new() };
    app_state.db.insert("foo".to_string(), "bar".to_string());
    let shared_app_state = Arc::new(RwLock::new(app_state));

    let app = Router::<SharedAppState>::new()
        .merge(api::v0::routes::create_routes())
        .with_state(Arc::clone(&shared_app_state));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:9000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}