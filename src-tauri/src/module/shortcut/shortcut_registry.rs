use crate::shared::app_handle::app_handle;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

pub struct ShortcutRegistry;

impl ShortcutRegistry {
    pub fn get_cancel_shortcut() -> Shortcut {
        Shortcut::new(Some(Modifiers::META | Modifiers::SHIFT), Code::KeyJ)
    }

    pub fn register_shortcuts() -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(desktop)]
        {
            app_handle()
                .global_shortcut()
                .register(Self::get_cancel_shortcut())?;
        }
        Ok(())
    }
}
