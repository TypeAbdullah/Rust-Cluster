use std::collections::HashMap;

use crate::db::models::{Deployment, GitHubAppConfig, GitHubInstallation, GitHubRepoInfo, Project};
use crate::db::Database;
use crate::error::AppError;

impl Database {
    // ── Projects ──────────────────────────────────────────────

    /// Create a new project.
    pub async fn create_project(&self, project: &Project) -> Result<(), AppError> {
        let conn = self.conn()?;
        let env_json = serde_json::to_string(&project.env_vars)?;
        conn.execute(
            "INSERT INTO projects (id, name, description, repo_url, branch, build_command, start_command, runtime, status, env_vars, auto_deploy, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            libsql::params![
                project.id.clone(),
                project.name.clone(),
                project.description.clone(),
                project.repo_url.clone(),
                project.branch.clone(),
                project.build_command.clone(),
                project.start_command.clone(),
                project.runtime.clone(),
                project.status.clone(),
                env_json,
                project.auto_deploy as i32,
                project.created_at.clone(),
                project.updated_at.clone(),
            ],
        )
        .await
        .map_err(|e| {
            if e.to_string().contains("UNIQUE") {
                AppError::Conflict(format!("Project '{}' already exists", project.name))
            } else {
                AppError::Database(e.to_string())
            }
        })?;
        Ok(())
    }

    /// List all projects.
    pub async fn list_projects(&self) -> Result<Vec<Project>, AppError> {
        let conn = self.conn()?;
        let mut rows = conn
            .query("SELECT * FROM projects ORDER BY created_at DESC", ())
            .await?;

        let mut projects = Vec::new();
        while let Some(row) = rows.next().await? {
            projects.push(row_to_project(&row)?);
        }
        Ok(projects)
    }

    /// Get a project by ID.
    pub async fn get_project(&self, id: &str) -> Result<Project, AppError> {
        let conn = self.conn()?;
        let mut rows = conn
            .query("SELECT * FROM projects WHERE id = ?1", libsql::params![id])
            .await?;

        match rows.next().await? {
            Some(row) => row_to_project(&row),
            None => Err(AppError::NotFound(format!("Project '{}' not found", id))),
        }
    }

    /// Update a project.
    pub async fn update_project(
        &self,
        id: &str,
        name: Option<&str>,
        description: Option<&str>,
        repo_url: Option<&str>,
        branch: Option<&str>,
        build_command: Option<&str>,
        start_command: Option<&str>,
        runtime: Option<&str>,
        auto_deploy: Option<bool>,
    ) -> Result<Project, AppError> {
        let project = self.get_project(id).await?;
        let now = chrono::Utc::now().to_rfc3339();

        let new_name = name.unwrap_or(&project.name);
        let new_desc = description.unwrap_or(&project.description);
        let new_repo = repo_url.unwrap_or(&project.repo_url);
        let new_branch = branch.unwrap_or(&project.branch);
        let new_build = build_command.unwrap_or(&project.build_command);
        let new_start = start_command.unwrap_or(&project.start_command);
        let new_runtime = runtime.unwrap_or(&project.runtime);
        let new_auto = auto_deploy.unwrap_or(project.auto_deploy);

        let conn = self.conn()?;
        conn.execute(
            "UPDATE projects SET name=?1, description=?2, repo_url=?3, branch=?4, build_command=?5, start_command=?6, runtime=?7, auto_deploy=?8, updated_at=?9 WHERE id=?10",
            libsql::params![new_name, new_desc, new_repo, new_branch, new_build, new_start, new_runtime, new_auto as i32, now, id],
        )
        .await?;

        self.get_project(id).await
    }

    /// Update project status.
    pub async fn update_project_status(&self, id: &str, status: &str) -> Result<(), AppError> {
        let conn = self.conn()?;
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE projects SET status=?1, updated_at=?2 WHERE id=?3",
            libsql::params![status, now, id],
        )
        .await?;
        Ok(())
    }

    /// Update project env vars.
    pub async fn update_project_env_vars(
        &self,
        id: &str,
        env_vars: &HashMap<String, String>,
    ) -> Result<(), AppError> {
        let conn = self.conn()?;
        let env_json = serde_json::to_string(env_vars)?;
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE projects SET env_vars=?1, updated_at=?2 WHERE id=?3",
            libsql::params![env_json, now, id],
        )
        .await?;
        Ok(())
    }

    /// Delete a project.
    pub async fn delete_project(&self, id: &str) -> Result<(), AppError> {
        self.get_project(id).await?;
        let conn = self.conn()?;
        conn.execute("DELETE FROM deployments WHERE project_id = ?1", libsql::params![id]).await?;
        conn.execute("DELETE FROM projects WHERE id = ?1", libsql::params![id]).await?;
        Ok(())
    }

    /// Get projects with a specific status.
    pub async fn get_projects_by_status(&self, status: &str) -> Result<Vec<Project>, AppError> {
        let conn = self.conn()?;
        let mut rows = conn
            .query("SELECT * FROM projects WHERE status = ?1", libsql::params![status])
            .await?;
        let mut projects = Vec::new();
        while let Some(row) = rows.next().await? {
            projects.push(row_to_project(&row)?);
        }
        Ok(projects)
    }

    // ── Deployments ───────────────────────────────────────────

    /// Create a new deployment.
    pub async fn create_deployment(&self, deployment: &Deployment) -> Result<(), AppError> {
        let conn = self.conn()?;
        conn.execute(
            "INSERT INTO deployments (id, project_id, commit_sha, commit_message, status, logs, trigger, created_at, finished_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            libsql::params![
                deployment.id.clone(),
                deployment.project_id.clone(),
                deployment.commit_sha.clone(),
                deployment.commit_message.clone(),
                deployment.status.clone(),
                deployment.logs.clone(),
                deployment.trigger.clone(),
                deployment.created_at.clone(),
                deployment.finished_at.clone(),
            ],
        )
        .await?;
        Ok(())
    }

    /// List deployments for a project.
    pub async fn list_deployments(&self, project_id: &str) -> Result<Vec<Deployment>, AppError> {
        let conn = self.conn()?;
        let mut rows = conn
            .query(
                "SELECT * FROM deployments WHERE project_id = ?1 ORDER BY created_at DESC LIMIT 50",
                libsql::params![project_id],
            )
            .await?;
        let mut deployments = Vec::new();
        while let Some(row) = rows.next().await? {
            deployments.push(row_to_deployment(&row)?);
        }
        Ok(deployments)
    }

    /// Get a deployment by ID.
    pub async fn get_deployment(&self, id: &str) -> Result<Deployment, AppError> {
        let conn = self.conn()?;
        let mut rows = conn
            .query("SELECT * FROM deployments WHERE id = ?1", libsql::params![id])
            .await?;
        match rows.next().await? {
            Some(row) => row_to_deployment(&row),
            None => Err(AppError::NotFound(format!("Deployment '{}' not found", id))),
        }
    }

    /// Update deployment status and logs.
    pub async fn update_deployment(&self, id: &str, status: &str, logs: &str, finished_at: Option<&str>) -> Result<(), AppError> {
        let conn = self.conn()?;
        let finished = finished_at.unwrap_or("");
        conn.execute(
            "UPDATE deployments SET status=?1, logs=?2, finished_at=?3 WHERE id=?4",
            libsql::params![status, logs, finished, id],
        )
        .await?;
        Ok(())
    }

    /// Append logs to a deployment.
    pub async fn append_deployment_logs(&self, id: &str, new_logs: &str) -> Result<(), AppError> {
        let conn = self.conn()?;
        conn.execute(
            "UPDATE deployments SET logs = logs || ?1 WHERE id = ?2",
            libsql::params![new_logs, id],
        )
        .await?;
        Ok(())
    }

    /// Count all deployments.
    pub async fn count_deployments(&self) -> Result<usize, AppError> {
        let conn = self.conn()?;
        let mut rows = conn.query("SELECT COUNT(*) FROM deployments", ()).await?;
        if let Some(row) = rows.next().await? {
            let count: i64 = row.get(0)?;
            return Ok(count as usize);
        }
        Ok(0)
    }

    // ── Settings (key-value) ──────────────────────────────────

    /// Get a setting by key.
    pub async fn get_setting(&self, key: &str) -> Result<Option<String>, AppError> {
        let conn = self.conn()?;
        let mut rows = conn
            .query("SELECT value FROM settings WHERE key = ?1", libsql::params![key])
            .await?;
        match rows.next().await? {
            Some(row) => {
                let val: String = row.get(0)?;
                Ok(Some(val))
            }
            None => Ok(None),
        }
    }

    /// Set a setting (upsert).
    pub async fn set_setting(&self, key: &str, value: &str) -> Result<(), AppError> {
        let conn = self.conn()?;
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO settings (key, value, updated_at) VALUES (?1, ?2, ?3)
             ON CONFLICT(key) DO UPDATE SET value=?2, updated_at=?3",
            libsql::params![key, value, now],
        )
        .await?;
        Ok(())
    }

    /// Delete a setting.
    pub async fn delete_setting(&self, key: &str) -> Result<(), AppError> {
        let conn = self.conn()?;
        conn.execute("DELETE FROM settings WHERE key = ?1", libsql::params![key]).await?;
        Ok(())
    }

    // ── GitHub App Config (stored as JSON in settings) ────────

    /// Get GitHub App config from settings.
    pub async fn get_github_app_config(&self) -> Result<Option<GitHubAppConfig>, AppError> {
        match self.get_setting("github_app").await? {
            Some(json) => {
                let config: GitHubAppConfig = serde_json::from_str(&json)?;
                Ok(Some(config))
            }
            None => Ok(None),
        }
    }

    /// Save GitHub App config to settings.
    pub async fn save_github_app_config(&self, config: &GitHubAppConfig) -> Result<(), AppError> {
        let json = serde_json::to_string(config)?;
        self.set_setting("github_app", &json).await
    }

    /// Delete GitHub App config.
    pub async fn delete_github_app_config(&self) -> Result<(), AppError> {
        self.delete_setting("github_app").await?;
        // Also clear all installations
        let conn = self.conn()?;
        conn.execute("DELETE FROM github_installations", ()).await?;
        Ok(())
    }

    // ── GitHub Installations ──────────────────────────────────

    /// Save or update a GitHub installation.
    pub async fn upsert_github_installation(&self, install: &GitHubInstallation) -> Result<(), AppError> {
        let conn = self.conn()?;
        let repos_json = serde_json::to_string(&install.repos)?;
        conn.execute(
            "INSERT INTO github_installations (installation_id, account_login, account_type, access_token, token_expires_at, repos_json, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
             ON CONFLICT(installation_id) DO UPDATE SET account_login=?2, account_type=?3, access_token=?4, token_expires_at=?5, repos_json=?6, updated_at=?8",
            libsql::params![
                install.installation_id.clone(),
                install.account_login.clone(),
                install.account_type.clone(),
                install.access_token.clone(),
                install.token_expires_at.clone(),
                repos_json,
                install.created_at.clone(),
                install.updated_at.clone(),
            ],
        )
        .await?;
        Ok(())
    }

    /// List all GitHub installations.
    pub async fn list_github_installations(&self) -> Result<Vec<GitHubInstallation>, AppError> {
        let conn = self.conn()?;
        let mut rows = conn
            .query("SELECT * FROM github_installations ORDER BY created_at DESC", ())
            .await?;
        let mut installs = Vec::new();
        while let Some(row) = rows.next().await? {
            installs.push(row_to_installation(&row)?);
        }
        Ok(installs)
    }

    /// Get a GitHub installation by ID.
    pub async fn get_github_installation(&self, installation_id: &str) -> Result<GitHubInstallation, AppError> {
        let conn = self.conn()?;
        let mut rows = conn
            .query(
                "SELECT * FROM github_installations WHERE installation_id = ?1",
                libsql::params![installation_id],
            )
            .await?;
        match rows.next().await? {
            Some(row) => row_to_installation(&row),
            None => Err(AppError::NotFound("Installation not found".to_string())),
        }
    }

    /// Delete a GitHub installation.
    pub async fn delete_github_installation(&self, installation_id: &str) -> Result<(), AppError> {
        let conn = self.conn()?;
        conn.execute(
            "DELETE FROM github_installations WHERE installation_id = ?1",
            libsql::params![installation_id],
        )
        .await?;
        Ok(())
    }
}

// ── Row converters ───────────────────────────────────────────

fn row_to_project(row: &libsql::Row) -> Result<Project, AppError> {
    let env_str: String = row.get::<String>(9).unwrap_or_else(|_| "{}".to_string());
    let env_vars: HashMap<String, String> = serde_json::from_str(&env_str).unwrap_or_default();
    let auto_deploy_int: i32 = row.get(10).unwrap_or(0);

    Ok(Project {
        id: row.get(0)?,
        name: row.get(1)?,
        description: row.get::<String>(2).unwrap_or_default(),
        repo_url: row.get::<String>(3).unwrap_or_default(),
        branch: row.get::<String>(4).unwrap_or_else(|_| "main".to_string()),
        build_command: row.get::<String>(5).unwrap_or_default(),
        start_command: row.get(6)?,
        runtime: row.get::<String>(7).unwrap_or_else(|_| "node".to_string()),
        status: row.get::<String>(8).unwrap_or_else(|_| "stopped".to_string()),
        env_vars,
        auto_deploy: auto_deploy_int != 0,
        created_at: row.get(11)?,
        updated_at: row.get(12)?,
    })
}

fn row_to_deployment(row: &libsql::Row) -> Result<Deployment, AppError> {
    Ok(Deployment {
        id: row.get(0)?,
        project_id: row.get(1)?,
        commit_sha: row.get::<String>(2).unwrap_or_default(),
        commit_message: row.get::<String>(3).unwrap_or_default(),
        status: row.get::<String>(4).unwrap_or_else(|_| "pending".to_string()),
        logs: row.get::<String>(5).unwrap_or_default(),
        trigger: row.get::<String>(6).unwrap_or_else(|_| "manual".to_string()),
        created_at: row.get(7)?,
        finished_at: row.get::<String>(8).unwrap_or_default(),
    })
}

fn row_to_installation(row: &libsql::Row) -> Result<GitHubInstallation, AppError> {
    let repos_json: String = row.get::<String>(5).unwrap_or_else(|_| "[]".to_string());
    let repos: Vec<GitHubRepoInfo> = serde_json::from_str(&repos_json).unwrap_or_default();

    Ok(GitHubInstallation {
        installation_id: row.get(0)?,
        account_login: row.get(1)?,
        account_type: row.get::<String>(2).unwrap_or_else(|_| "User".to_string()),
        access_token: row.get::<String>(3).unwrap_or_default(),
        token_expires_at: row.get::<String>(4).unwrap_or_default(),
        repos,
        created_at: row.get(6)?,
        updated_at: row.get(7)?,
    })
}
