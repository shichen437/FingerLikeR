use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::time::Duration;
use tauri::{AppHandle, Wry};
use tauri_plugin_store::StoreExt;
use tokio::task::JoinHandle;
use tokio::time;

use super::mouse;

static IDLE_MOVE_JOB_HANDLE: Lazy<Mutex<Option<JoinHandle<()>>>> = Lazy::new(|| Mutex::new(None));

pub fn toggle_idle_move_job(app_handle: AppHandle<Wry>) -> Result<(), String> {
    let store = app_handle
        .store("r_store.json")
        .map_err(|e| format!("Failed to get store: {}", e))?;

    let idle_move_value = store
        .get("task.idleMove")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);

    let mut job_handle_guard = IDLE_MOVE_JOB_HANDLE.lock().unwrap();

    if idle_move_value == 1 {
        if job_handle_guard.is_none() {
            let task_app_handle = app_handle.clone();
            let handle = tokio::spawn(async move {
                let mut interval = time::interval(Duration::from_secs(300));
                loop {
                    interval.tick().await;
                    mouse::move_mouse_randomly_smooth(task_app_handle.clone());
                }
            });
            *job_handle_guard = Some(handle);
        }
    } else {
        if let Some(handle) = job_handle_guard.take() {
            handle.abort();
        }
    }

    Ok(())
}
