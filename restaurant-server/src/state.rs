use std::{collections::HashMap, sync::{Arc, RwLock}};

pub type SharedAppState = Arc<RwLock<AppState>>;

pub struct AppState {
    pub db: HashMap<String, String>,
}
