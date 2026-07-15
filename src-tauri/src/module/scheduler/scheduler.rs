use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time;

use crate::core::mouse::mouse_move;
use crate::infra::store::get_i64;

static IDLE_MOVE_JOB_HANDLE: Lazy<Mutex<Option<JoinHandle<()>>>> = Lazy::new(|| Mutex::new(None));

pub fn toggle_idle_move_job() -> Result<(), String> {
    let idle_move_value = get_i64("task.idleMove", 0);

    let mut job_handle_guard = IDLE_MOVE_JOB_HANDLE.lock().unwrap();

    if idle_move_value == 1 {
        if job_handle_guard.is_none() {
            let handle = tokio::spawn(async move {
                let mut interval = time::interval(Duration::from_secs(300));
                loop {
                    interval.tick().await;
                    mouse_move::move_mouse_randomly_smooth();
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
