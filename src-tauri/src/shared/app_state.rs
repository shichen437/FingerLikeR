use crate::core::mouse::model::click_task::{TaskProgress, TaskStatus};
use std::sync::{Arc, Mutex};
use tauri::{Emitter, WebviewWindow};

#[derive(Clone)]
pub struct AppState {
    pub task_progress: Arc<Mutex<TaskProgress>>,
    pub is_running: Arc<Mutex<bool>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            task_progress: Arc::new(Mutex::new(TaskProgress::default())),
            is_running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn set_status(&self, window: &WebviewWindow, status: TaskStatus) {
        let mut progress = self.task_progress.lock().unwrap();
        progress.status = status;
        window.emit("task-progress", progress.clone()).unwrap();
    }

    pub fn set_progress(&self, window: &WebviewWindow, status: TaskStatus, prog: u32, total: u32) {
        let mut progress = self.task_progress.lock().unwrap();
        progress.status = status;
        progress.progress = prog;
        progress.total = total;
        window.emit("task-progress", progress.clone()).unwrap();
    }

    pub fn get_progress(&self) -> TaskProgress {
        self.task_progress.lock().unwrap().clone()
    }

    pub fn start_task(&self) {
        let mut running = self.is_running.lock().unwrap();
        *running = true;
    }

    pub fn finish_task(&self) {
        let mut running = self.is_running.lock().unwrap();
        *running = false;
        self.task_progress.lock().unwrap().status = TaskStatus::Idle;
    }

    pub fn is_running(&self) -> bool {
        *self.is_running.lock().unwrap()
    }
}
