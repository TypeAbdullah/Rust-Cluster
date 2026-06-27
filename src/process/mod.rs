pub mod builder;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::RwLock;

use crate::db::Database;

/// Information about a managed process.
#[derive(Debug)]
pub struct ManagedProcess {
    pub project_id: String,
    pub pid: Option<u32>,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub restart_count: u32,
    cancel_tx: tokio::sync::watch::Sender<bool>,
}

/// Manages all running bot/app processes.
pub struct ProcessManager {
    pub processes: Arc<RwLock<HashMap<String, ManagedProcess>>>,
    pub apps_dir: String,
    pub db: Arc<Database>,
}

impl ProcessManager {
    pub fn new(apps_dir: String, db: Arc<Database>) -> Self {
        // Ensure apps directory exists
        std::fs::create_dir_all(&apps_dir).ok();

        Self {
            processes: Arc::new(RwLock::new(HashMap::new())),
            apps_dir,
            db,
        }
    }

    /// Restore projects that were previously in "running" state.
    pub async fn restore_running_projects(&self) {
        match self.db.get_projects_by_status("running").await {
            Ok(projects) => {
                for project in projects {
                    tracing::info!("🔄 Restoring project: {}", project.name);
                    if let Err(e) = self.start_project(&project.id).await {
                        tracing::error!("Failed to restore project {}: {}", project.name, e);
                        self.db.update_project_status(&project.id, "crashed").await.ok();
                    }
                }
            }
            Err(e) => tracing::error!("Failed to query running projects: {}", e),
        }
    }

    /// Start a project's process.
    pub async fn start_project(&self, project_id: &str) -> Result<(), crate::AppError> {
        // Check if already running
        {
            let processes = self.processes.read().await;
            if processes.contains_key(project_id) {
                return Err(crate::AppError::Conflict("Project is already running".to_string()));
            }
        }

        let project = self.db.get_project(project_id).await?;

        // Determine working directory
        let work_dir = if !project.repo_url.is_empty() {
            let project_dir = format!("{}/{}", self.apps_dir, project.name);
            std::path::Path::new(&project_dir).to_path_buf()
        } else {
            std::path::Path::new(&self.apps_dir).to_path_buf()
        };

        // Parse start command
        let parts: Vec<&str> = project.start_command.split_whitespace().collect();
        if parts.is_empty() {
            return Err(crate::AppError::BadRequest("Empty start command".to_string()));
        }

        let (program, args) = if cfg!(target_os = "windows") {
            ("cmd".to_string(), vec!["/C".to_string(), project.start_command.clone()])
        } else {
            ("sh".to_string(), vec!["-c".to_string(), project.start_command.clone()])
        };

        // Spawn the process
        let mut cmd = Command::new(&program);
        cmd.args(&args)
            .current_dir(&work_dir)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .kill_on_drop(true);

        // Set environment variables
        for (key, value) in &project.env_vars {
            cmd.env(key, value);
        }

        let mut child = cmd.spawn().map_err(|e| {
            crate::AppError::Internal(format!("Failed to spawn process: {}", e))
        })?;

        let pid = child.id();
        let (cancel_tx, cancel_rx) = tokio::sync::watch::channel(false);

        // Store process info
        {
            let mut processes = self.processes.write().await;
            processes.insert(
                project_id.to_string(),
                ManagedProcess {
                    project_id: project_id.to_string(),
                    pid,
                    started_at: chrono::Utc::now(),
                    restart_count: 0,
                    cancel_tx,
                },
            );
        }

        // Update status in DB
        self.db.update_project_status(project_id, "running").await?;

        // Spawn background task to monitor the process
        let processes = self.processes.clone();
        let db = self.db.clone();
        let pid_str = project_id.to_string();
        let project_name = project.name.clone();

        tokio::spawn(async move {
            Self::monitor_process(child, &pid_str, &project_name, processes, db, cancel_rx).await;
        });

        tracing::info!("✅ Started project '{}' (PID: {:?})", project.name, pid);
        Ok(())
    }

    /// Monitor a running process, capture output, handle crashes.
    async fn monitor_process(
        mut child: Child,
        project_id: &str,
        project_name: &str,
        processes: Arc<RwLock<HashMap<String, ManagedProcess>>>,
        db: Arc<Database>,
        mut cancel_rx: tokio::sync::watch::Receiver<bool>,
    ) {
        // Capture stdout
        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            let db_out = db.clone();
            let pid_out = project_id.to_string();
            let name_out = project_name.to_string();

            tokio::spawn(async move {
                while let Ok(Some(line)) = lines.next_line().await {
                    tracing::info!("[{}] {}", name_out, line);
                    let _ = db_out; // logs are traced; could store in DB if needed
                }
            });
        }

        // Capture stderr
        if let Some(stderr) = child.stderr.take() {
            let reader = BufReader::new(stderr);
            let mut lines = reader.lines();
            let name_err = project_name.to_string();

            tokio::spawn(async move {
                while let Ok(Some(line)) = lines.next_line().await {
                    tracing::warn!("[{}] stderr: {}", name_err, line);
                }
            });
        }

        // Wait for process to exit or cancellation
        tokio::select! {
            status = child.wait() => {
                match status {
                    Ok(exit) => {
                        tracing::warn!(
                            "Process '{}' exited with status: {}",
                            project_name,
                            exit
                        );
                    }
                    Err(e) => {
                        tracing::error!("Error waiting for process '{}': {}", project_name, e);
                    }
                }

                // Remove from tracking and update status
                let mut procs = processes.write().await;
                procs.remove(project_id);
                db.update_project_status(project_id, "stopped").await.ok();
            }
            _ = cancel_rx.changed() => {
                // Process was cancelled (stop requested)
                tracing::info!("Stopping process '{}' (cancel signal received)", project_name);
                child.kill().await.ok();
                let mut procs = processes.write().await;
                procs.remove(project_id);
                db.update_project_status(project_id, "stopped").await.ok();
            }
        }
    }

    /// Stop a project's process.
    pub async fn stop_project(&self, project_id: &str) -> Result<(), crate::AppError> {
        let mut processes = self.processes.write().await;
        match processes.remove(project_id) {
            Some(managed) => {
                // Send cancel signal
                managed.cancel_tx.send(true).ok();
                self.db.update_project_status(project_id, "stopped").await?;
                tracing::info!("🛑 Stopped project '{}'", project_id);
                Ok(())
            }
            None => Err(crate::AppError::NotFound("Project is not running".to_string())),
        }
    }

    /// Restart a project's process.
    pub async fn restart_project(&self, project_id: &str) -> Result<(), crate::AppError> {
        // Stop if running (ignore error if not running)
        self.stop_project(project_id).await.ok();
        // Brief pause for cleanup
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        self.start_project(project_id).await
    }

    /// Check if a project is currently running.
    pub async fn is_running(&self, project_id: &str) -> bool {
        let processes = self.processes.read().await;
        processes.contains_key(project_id)
    }

    /// Get info about a running process.
    pub async fn get_process_info(&self, project_id: &str) -> Option<ProcessInfo> {
        let processes = self.processes.read().await;
        processes.get(project_id).map(|p| ProcessInfo {
            pid: p.pid,
            started_at: p.started_at.to_rfc3339(),
            restart_count: p.restart_count,
            uptime_seconds: (chrono::Utc::now() - p.started_at).num_seconds() as u64,
        })
    }

    /// Get count of running processes.
    pub async fn running_count(&self) -> usize {
        let processes = self.processes.read().await;
        processes.len()
    }
}

/// Serializable process info.
#[derive(Debug, serde::Serialize)]
pub struct ProcessInfo {
    pub pid: Option<u32>,
    pub started_at: String,
    pub restart_count: u32,
    pub uptime_seconds: u64,
}
