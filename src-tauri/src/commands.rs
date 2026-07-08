use crate::core::{
    accessibility,
    models::task::{TaskProgress, TaskStatus},
    mouse, scheduler,
    state::AppState,
};

#[tauri::command]
pub fn is_accessibility_permission_granted() -> bool {
    accessibility::is_accessibility_enabled()
}

#[tauri::command]
pub fn start_clicking_task(
    app: tauri::AppHandle,
    window: tauri::WebviewWindow,
    count: u32,
    state: tauri::State<AppState>,
) {
    if state.is_running() {
        return;
    }
    state.start_task();
    mouse::start_clicking(app, window, count, state.inner().clone());
}

#[tauri::command]
pub fn get_mouse_location() -> (i32, i32) {
    mouse::get_location()
}

#[tauri::command]
pub fn cancel_task(window: tauri::WebviewWindow, state: tauri::State<AppState>) {
    if state.is_running() {
        state.finish_task();
        state.set_status(&window, TaskStatus::Cancelled);
    }
}

#[tauri::command]
pub fn get_task_status(state: tauri::State<AppState>) -> TaskProgress {
    state.get_progress()
}

#[tauri::command]
pub async fn toggle_idle_move_job(app_handle: tauri::AppHandle) -> Result<(), String> {
    scheduler::toggle_idle_move_job(app_handle)
}
