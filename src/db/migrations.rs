use libsql::Connection;

/// Run all database migrations in order.
pub async fn run_all(conn: &Connection) -> Result<(), libsql::Error> {
    // Projects table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            description TEXT DEFAULT '',
            repo_url TEXT DEFAULT '',
            branch TEXT DEFAULT 'main',
            build_command TEXT DEFAULT '',
            start_command TEXT NOT NULL,
            runtime TEXT DEFAULT 'node',
            status TEXT DEFAULT 'stopped',
            env_vars TEXT DEFAULT '{}',
            auto_deploy INTEGER DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        (),
    )
    .await?;

    // Deployments table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS deployments (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            commit_sha TEXT DEFAULT '',
            commit_message TEXT DEFAULT '',
            status TEXT DEFAULT 'pending',
            logs TEXT DEFAULT '',
            trigger TEXT DEFAULT 'manual',
            created_at TEXT NOT NULL,
            finished_at TEXT DEFAULT '',
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
        )",
        (),
    )
    .await?;

    // Settings table (key-value store for GitHub App creds, etc.)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        (),
    )
    .await?;

    // GitHub installations table (tracks which repos are accessible)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS github_installations (
            installation_id TEXT PRIMARY KEY,
            account_login TEXT NOT NULL,
            account_type TEXT DEFAULT 'User',
            access_token TEXT DEFAULT '',
            token_expires_at TEXT DEFAULT '',
            repos_json TEXT DEFAULT '[]',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        (),
    )
    .await?;

    // Indexes
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_deployments_project_id ON deployments(project_id)",
        (),
    )
    .await?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_deployments_created_at ON deployments(created_at DESC)",
        (),
    )
    .await?;

    tracing::info!("✅ All migrations completed");
    Ok(())
}
