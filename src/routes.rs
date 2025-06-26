use axum::{routing::get, Router};
use bb8_redis::{bb8, RedisConnectionManager};
use std::sync::Arc;

use crate::handlers;
use crate::middleware;

pub async fn build_router(redis_url: &str) -> Result<Router, Box<dyn std::error::Error>> {
    // Initialize Redis connection pool
    let manager = RedisConnectionManager::new(redis_url)?;

    let pool = bb8::Pool::builder().build(manager).await?;
    let pool_arc = Arc::new(pool);

    // Build our application with routes
    let app = Router::new()
        .route("/", get(handlers::hello_world))
        .layer(middleware::create_trace_layer())
        .with_state(pool_arc);

    Ok(app)
}
