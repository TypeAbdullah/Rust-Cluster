use std::sync::Arc;
use tokio::process::Command;

use crate::db::Database;
use crate::error::AppError;

/// Build and deploy a project from its GitHub repository.
pub async fn build_and_deploy(
    project_id: &str,
    deployment_id: &str,
    db: Arc<Database>,
    apps_dir: &str,
) -> Result<(), AppError> {
    let project = db.get_project(project_id).await?;
    let project_dir = format!("{}/{}", apps_dir, project.name);

    // Update deployment status to building
    db.update_deployment(deployment_id, "building", "", None)
        .await?;

    let mut all_logs = String::new();

    // Step 1: Clone or pull the repository
    if !project.repo_url.is_empty() {
        let log_line = format!("📦 Cloning repository: {}\n", project.repo_url);
        all_logs.push_str(&log_line);
        db.append_deployment_logs(deployment_id, &log_line).await?;

        if std::path::Path::new(&project_dir).exists() {
            // Pull latest
            let output = run_command_in_dir("git", &["pull", "origin", &project.branch], &project_dir).await?;
            all_logs.push_str(&output);
            db.append_deployment_logs(deployment_id, &output).await?;
        } else {
            // Clone
            let output = run_command_in_dir(
                "git",
                &["clone", "-b", &project.branch, &project.repo_url, &project_dir],
                apps_dir,
            )
            .await?;
            all_logs.push_str(&output);
            db.append_deployment_logs(deployment_id, &output).await?;
        }

        // Get latest commit info
        if let Ok(sha) = run_command_in_dir("git", &["rev-parse", "--short", "HEAD"], &project_dir).await {
            let sha = sha.trim().to_string();
            let msg = run_command_in_dir("git", &["log", "-1", "--pretty=%B"], &project_dir)
                .await
                .unwrap_or_default()
                .trim()
                .to_string();

            // Update deployment with commit info
            let conn = db.conn()?;
            conn.execute(
                "UPDATE deployments SET commit_sha=?1, commit_message=?2 WHERE id=?3",
                libsql::params![sha, msg, deployment_id],
            )
            .await?;
        }
    }

    // Step 2: Run build command
    if !project.build_command.is_empty() {
        let log_line = format!("🔨 Running build: {}\n", project.build_command);
        all_logs.push_str(&log_line);
        db.append_deployment_logs(deployment_id, &log_line).await?;

        let work_dir = if std::path::Path::new(&project_dir).exists() {
            &project_dir
        } else {
            apps_dir
        };

        let output = run_shell_command(&project.build_command, work_dir).await?;
        all_logs.push_str(&output);
        db.append_deployment_logs(deployment_id, &output).await?;
    }

    // Step 3: Mark deployment as successful
    let finish_log = "✅ Build completed successfully\n".to_string();
    all_logs.push_str(&finish_log);
    db.append_deployment_logs(deployment_id, &finish_log).await?;

    let now = chrono::Utc::now().to_rfc3339();
    db.update_deployment(deployment_id, "success", &all_logs, Some(&now))
        .await?;

    tracing::info!("✅ Build completed for project '{}'", project.name);
    Ok(())
}

/// Run a command with specific args in a directory.
async fn run_command_in_dir(program: &str, args: &[&str], dir: &str) -> Result<String, AppError> {
    let output = Command::new(program)
        .args(args)
        .current_dir(dir)
        .output()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to run {}: {}", program, e)))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    let mut result = String::new();
    if !stdout.is_empty() {
        result.push_str(&stdout);
    }
    if !stderr.is_empty() {
        result.push_str(&stderr);
    }

    if !output.status.success() {
        return Err(AppError::Internal(format!(
            "Command failed with exit code {:?}:\n{}",
            output.status.code(),
            result
        )));
    }

    Ok(result)
}

/// Run a shell command string in a directory.
async fn run_shell_command(command: &str, dir: &str) -> Result<String, AppError> {
    let (shell, flag) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    run_command_in_dir(shell, &[flag, command], dir).await
}
