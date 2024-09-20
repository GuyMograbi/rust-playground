use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers;

pub fn routes() -> Router {
    Router::new()
        .route(
            "/hello-world",
            get(handlers::hello_world)
        )
        .route(
            "/users",
            post(handlers::create_user)
        )
        .layer(axum::middleware::from_fn(
            crate::middleware::logging_middleware,
        ))
}
