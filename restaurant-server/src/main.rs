use app::create_app;
use persistence::memory_persistence::MemoryPersistence;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod app;
mod models;
mod persistence;
mod state;

#[cfg(test)]
mod tests {
    mod app_integration_tests;
    mod memory_persistence_tests;
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let persistence: MemoryPersistence = MemoryPersistence::default();
    let app = create_app(persistence);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:9000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
