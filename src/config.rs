/// Application configuration loaded from environment variables.
/// GitHub App credentials are NOT here — they live in the database,
/// configured via the frontend UI (Settings → GitHub).
#[derive(Debug, Clone)]
pub struct AppConfig {
    // Admin auth
    pub admin_email: String,
    pub admin_password: String,
    pub admin_username: String,
    pub jwt_secret: String,

    // Server
    pub port: u16,
    pub apps_dir: String,
    pub app_url: String,
}

impl AppConfig {
    /// Load configuration from environment variables with sensible defaults.
    pub fn from_env() -> Self {
        Self {
            admin_email: std::env::var("ADMIN_EMAIL")
                .unwrap_or_else(|_| "admin@rustcluster.local".to_string()),
            admin_password: std::env::var("ADMIN_PASSWORD")
                .unwrap_or_else(|_| "admin".to_string()),
            admin_username: std::env::var("ADMIN_USERNAME")
                .unwrap_or_else(|_| "admin".to_string()),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "rustcluster-default-secret-change-me-in-production".to_string()),
            port: std::env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
            apps_dir: std::env::var("APPS_DIR")
                .unwrap_or_else(|_| "./apps".to_string()),
            app_url: std::env::var("APP_URL")
                .unwrap_or_else(|_| "http://localhost:8080".to_string()),
        }
    }
}
