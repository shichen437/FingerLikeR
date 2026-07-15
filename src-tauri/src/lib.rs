mod bootstrap;
mod core;
mod infra;
mod module;
mod shared;

use crate::shared::app_state::AppState;
use tauri::async_runtime;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::new();
    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(module::shortcut::ShortcutHandler::create_handler())
                .build(),
        )
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            shared::app_handle::set_app_handle(app.handle().clone());
            infra::store::init_store()?;
            module::trayicon::trayicon::init_tray_menu()?;
            async_runtime::spawn(async move {
                let _ = module::scheduler::scheduler::toggle_idle_move_job();
            });

            #[cfg(desktop)]
            {
                module::shortcut::ShortcutRegistry::register_shortcuts()?;
            }

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            bootstrap::commands::start_clicking_task,
            bootstrap::commands::get_mouse_location,
            bootstrap::commands::get_task_status,
            bootstrap::commands::toggle_idle_move_job,
            bootstrap::commands::is_accessibility_permission_granted
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
