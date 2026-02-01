use crate::engine::logs::cleanup_all_logs;
use crate::storage::{self, models::ProjectStatus};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Recovery action for interrupted tasks
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecoveryInfo {
    pub project_id: String,
    pub project_name: String,
    pub iteration: u32,
    pub status: String,
}

/// Check for interrupted tasks on startup
#[tauri::command]
pub async fn check_interrupted_tasks() -> Result<Vec<RecoveryInfo>, String> {
    let index = storage::load_project_index().map_err(|e| e.to_string())?;
    let mut interrupted = Vec::new();

    for project_meta in &index.projects {
        if let Ok(state) = storage::load_project_state(&project_meta.id) {
            match state.status {
                ProjectStatus::Running | ProjectStatus::Pausing => {
                    let iteration = state
                        .execution
                        .as_ref()
                        .map(|e| e.current_iteration)
                        .unwrap_or(0);

                    interrupted.push(RecoveryInfo {
                        project_id: state.id.to_string(),
                        project_name: state.name.clone(),
                        iteration,
                        status: format!("{:?}", state.status).to_lowercase(),
                    });
                }
                _ => {}
            }
        }
    }

    Ok(interrupted)
}

/// Mark an interrupted task as cancelled
#[tauri::command]
pub async fn cancel_interrupted_task(project_id: String) -> Result<(), String> {
    let uuid = Uuid::parse_str(&project_id).map_err(|e| e.to_string())?;
    let mut state = storage::load_project_state(&uuid).map_err(|e| e.to_string())?;

    state.status = ProjectStatus::Cancelled;
    state.updated_at = chrono::Utc::now();

    storage::save_project_state(&state).map_err(|e| e.to_string())?;

    Ok(())
}

/// Clean up old logs based on retention policy
#[tauri::command]
pub async fn cleanup_logs() -> Result<u32, String> {
    let config = storage::load_config().map_err(|e| e.to_string())?;
    cleanup_all_logs(config.log_retention_days)
}
