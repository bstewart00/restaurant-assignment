use std::sync::Arc;

use tokio::sync::RwLock;

use crate::persistence::memory_persistence::MemoryPersistence;

// This ultimately means the whole hashmap is locked during writes, even for readers wanting to read unrelated keys
// For this demo it's probably not worth, and perhaps a real restaurant might be OK with this too.
pub type SharedAppState = Arc<RwLock<AppState>>;

// For simplicitly i'm not going to try and unravel async traits and Box<dyn Persistence>
pub struct AppState {
    pub persistence: MemoryPersistence
}
