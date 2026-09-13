use std::sync::Arc;

use application::ApplicationsImpl;
use common::{BoxError, config};
use domain::Repositories;
use infrastructure::RepositoriesImpl;
use presentation::create_router;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    if let Some(level) = config().logging.level.clone() {
        tracing_subscriber::fmt().with_env_filter(level).init();
    }

    let repos = Arc::new(RepositoriesImpl::new());
    let applications = ApplicationsImpl::new(repos.clone());

    let app = create_router(Arc::new(applications));

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config().server.host, config().server.port))
            .await?;
    tracing::info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("Shutting down gracefully, please wait.");
    if let Some(repos) = Arc::into_inner(repos) {
        repos.flush().await;
        let _ = repos.user().commit().await;
        let _ = repos.todo().commit().await;
        repos.flush().await;
    }
    tracing::info!("Shutdown complete.");

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
