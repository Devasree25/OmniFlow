use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
mod middleware;
mod models;
mod routes;
mod services;
mod utils;

use routes::{auth, workflows, users};

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub jwt_secret: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "omniflow_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Database connection
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Connected to database successfully");

    // JWT secret
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    // Application state
    let state = AppState {
        db: db_pool,
        jwt_secret,
    };

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build routes
    let app = Router::new()
        // Health check
        .route("/health", get(health_check))
        
        // Auth routes
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/me", get(auth::get_current_user))
        
        // User routes (protected)
        .route("/api/users", get(users::list_users))
        .route("/api/users/:id", get(users::get_user))
        
        // Workflow routes (protected)
        .route("/api/workflows", get(workflows::list_workflows))
        .route("/api/workflows", post(workflows::create_workflow))
        .route("/api/workflows/:id", get(workflows::get_workflow))
        .route("/api/workflows/:id/approve", post(workflows::approve_workflow))
        .route("/api/workflows/:id/reject", post(workflows::reject_workflow))
        
        // Workflow configs
        .route("/api/workflow-configs", get(workflows::list_workflow_configs))
        .route("/api/workflow-configs", post(workflows::create_workflow_config))
        
        .layer(cors)
        .with_state(state);

    // Server configuration
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("{}:{}", host, port);

    tracing::info!("🚀 Server starting on {}", addr);

    // Start server
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
