use mes_gateway::app;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "mes_gateway=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    
    tracing::info!("MES Gateway listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}