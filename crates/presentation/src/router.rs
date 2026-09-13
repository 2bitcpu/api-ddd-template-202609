use crate::{
    handler::{auth, todo},
    middleware::auth::auth_guard,
};
use application::Applications;
use axum::{
    Router,
    extract::DefaultBodyLimit,
    http::{HeaderValue, header},
    middleware::from_fn_with_state,
    routing::{any, delete, get, get_service, post, put},
};
use common::config;
use std::sync::Arc;
use tower_http::{
    cors::{AllowOrigin, Any, CorsLayer},
    services::ServeDir,
};

#[allow(dead_code)]
pub fn create_router(usecases: Arc<dyn Applications>) -> Router {
    let todo_router = Router::new()
        .route("/todo", put(todo::entry))
        .route("/todo", post(todo::replace))
        .route("/todo/{id}", delete(todo::remove))
        .route("/todo", get(todo::list))
        .layer(from_fn_with_state(usecases.clone(), auth_guard));

    let mut auth_router = Router::new()
        .route("/auth/signin", post(auth::signin))
        .route("/auth/signout", any(auth::signout));

    if config().security.allow_signup {
        tracing::info!("allow signup");
        auth_router = auth_router.route("/auth/signup", post(auth::signup));
    }

    let mut app = Router::new()
        .nest("/service", todo_router)
        .nest("/service", auth_router)
        .with_state(usecases);

    if !config().server.cors.is_empty() {
        if config().server.cors.iter().any(|origin| origin == "*") {
            tracing::info!("allow any origin");
            app = app.layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]),
            );
        } else {
            tracing::info!("allow specific origins");
            let origins = config()
                .server
                .cors
                .iter()
                .filter_map(|origin| HeaderValue::try_from(origin).ok())
                .collect::<Vec<_>>();

            app = app.layer(
                CorsLayer::new()
                    .allow_origin(AllowOrigin::list(origins))
                    .allow_methods(Any)
                    .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]),
            );
        }
    }

    app = app.layer(DefaultBodyLimit::max(1024 * 1024));

    if let Some(dir) = config().server.static_dir.as_ref() {
        tracing::info!("static dir: {}", dir.display());
        app.fallback(get_service(ServeDir::new(dir)))
    } else {
        app
    }
}
