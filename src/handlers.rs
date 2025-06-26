use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use bb8_redis::{bb8, redis::AsyncCommands, RedisConnectionManager};
use std::sync::Arc;
use tracing::Instrument;

pub async fn hello_world(
    axum::extract::State(pool): axum::extract::State<Arc<bb8::Pool<RedisConnectionManager>>>,
) -> Response {
    // Get a connection from the pool
    let mut conn = match pool.get().await {
        Ok(conn) => {
            tracing::debug!("Successfully obtained Redis connection from pool");
            conn
        }
        Err(_) => {
            tracing::error!("Failed to get database connection from pool");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get database connection",
            )
                .into_response();
        }
    };

    // Ideally we'd have an InstrumentedAsyncCommands trait to use which
    // generates spans automatically, but here we are.
    let span = tracing::info_span!(
        "INCRBY visit_counter",
        db.system = "redis",
        db.operation.name = "INCRBY",
        db.collection.name = "visit_counter",
        db.statement = "INCRBY visit_counter 1",
    );

    // Increment the visit counter in Redis
    let visit_count: i32 = match conn.incr("visit_counter", 1).instrument(span).await {
        Ok(count) => count,
        Err(_) => {
            tracing::error!("Failed to increment visit counter in Redis");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to increment visit counter",
            )
                .into_response();
        }
    };

    (
        StatusCode::OK,
        format!("Hello, World! You are visitor number {}", visit_count),
    )
        .into_response()
}
