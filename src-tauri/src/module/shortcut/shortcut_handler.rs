use crate::bootstrap::commands;
use crate::shared::app_state::AppState;
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutEvent};

pub struct ShortcutHandler;

impl ShortcutHandler {
    pub fn create_handler() -> impl Fn(&AppHandle, &Shortcut, ShortcutEvent) {
        |app_handle, shortcut, event| {
            let cancel_shortcut =
                Shortcut::new(Some(Modifiers::META | Modifiers::SHIFT), Code::KeyJ);

            if shortcut == &cancel_shortcut {
                if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                    if let Some(main_window) = app_handle.get_webview_window("main") {
                        let app_state = app_handle.state::<AppState>();
                        commands::cancel_task(main_window, app_state);
                    }
                }
            }
        }
    }
}
