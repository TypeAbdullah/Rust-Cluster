use std::sync::Arc;

use axum::{
    http::{header, Method, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    Router,
};
use rust_embed::Embed;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod config;
mod db;
mod error;
mod github;
mod process;
mod routes;

pub use config::AppConfig;
pub use error::AppError;

/// Shared application state passed to all handlers.
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<db::Database>,
    pub config: Arc<AppConfig>,
    pub process_manager: Arc<process::ProcessManager>,
}

/// Embedded frontend assets — compiled into the binary.
#[derive(Embed)]
#[folder = "frontend/"]
struct FrontendAssets;

#[tokio::main]
async fn main() {
    // Load .env file if present
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rustcluster=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("🚀 Starting RustCluster...");

    // Load configuration
    let config = AppConfig::from_env();
    let port = config.port;

    // Initialize local database (auto-creates rustcluster.db)
    let database = db::Database::init()
        .await
        .expect("Failed to initialize database");
    database
        .run_migrations()
        .await
        .expect("Failed to run migrations");
    tracing::info!("✅ Database initialized (local SQLite)");

    // Initialize process manager
    let process_manager =
        process::ProcessManager::new(config.apps_dir.clone(), Arc::new(database.clone()));

    // Restore previously running projects
    process_manager.restore_running_projects().await;
    tracing::info!("✅ Process manager ready");

    // Build shared state
    let state = AppState {
        db: Arc::new(database),
        config: Arc::new(config),
        process_manager: Arc::new(process_manager),
    };

    // CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    // Build router
    let app = Router::new()
        // API routes (includes public + protected + webhooks)
        .nest("/api", routes::api_router(state.clone()))
        // Health check
        .route("/health", axum::routing::get(|| async { "OK" }))
        // Frontend SPA fallback
        .fallback(static_handler)
        .with_state(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // Start server
    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("🌐 Dashboard: http://localhost:{}", port);
    tracing::info!("📡 API: http://localhost:{}/api", port);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}

/// Serve embedded frontend files, with SPA fallback to index.html.
async fn static_handler(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');

    // Try to serve the exact file
    if let Some(file) = FrontendAssets::get(path) {
        let mime = mime_guess::from_path(path).first_or_octet_stream();
        return (
            StatusCode::OK,
            [(header::CONTENT_TYPE, mime.as_ref())],
            file.data.to_vec(),
        )
            .into_response();
    }

    // SPA fallback: serve index.html for all non-file routes
    match FrontendAssets::get("index.html") {
        Some(file) => Html(String::from_utf8_lossy(&file.data).to_string()).into_response(),
        None => (StatusCode::NOT_FOUND, "Frontend not found").into_response(),
    }
}
