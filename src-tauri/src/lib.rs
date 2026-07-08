mod commands;
mod core;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

use crate::core::state::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::new();
    tauri::Builder::default()
        .manage(app_state)
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app_handle, shortcut, event| {
                    let cmd_shift_j =
                        Shortcut::new(Some(Modifiers::META | Modifiers::SHIFT), Code::KeyJ);

                    if shortcut == &cmd_shift_j {
                        if let ShortcutState::Pressed = event.state() {
                            if let Some(main_window) = app_handle.get_webview_window("main") {
                                let app_state = app_handle.state::<AppState>();
                                commands::cancel_task(main_window, app_state);
                            }
                        }
                    }
                })
                .build(),
        )
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            core::store::init_store(app.handle())?;

            if let Err(e) = core::scheduler::toggle_idle_move_job(app.handle().clone()) {
                eprintln!("Failed to toggle idle move job on startup: {}", e);
            }

            #[cfg(desktop)]
            {
                let cmd_shift_j =
                    Shortcut::new(Some(Modifiers::META | Modifiers::SHIFT), Code::KeyJ);
                app.global_shortcut().register(cmd_shift_j)?;
            }

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::start_clicking_task,
            commands::get_mouse_location,
            commands::get_task_status,
            commands::toggle_idle_move_job,
            commands::is_accessibility_permission_granted
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
