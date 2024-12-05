use std::sync::Arc;

use axum::Router;
use tokio::sync::RwLock;

use crate::{
    api,
    persistence::memory_persistence::MemoryPersistence,
    state::{AppState, SharedAppState},
};

pub fn create_app(persistence: MemoryPersistence) -> Router {
    let app_state = AppState { persistence: persistence };
    let shared_app_state = Arc::new(RwLock::new(app_state));

    return Router::<SharedAppState>::new()
        .merge(api::v0::routes::create_routes())
        .with_state(Arc::clone(&shared_app_state));
}
