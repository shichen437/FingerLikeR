use serde_json::json;
use tauri::{AppHandle, Wry};
use tauri_plugin_store::StoreExt;

pub fn init_store(app: &AppHandle<Wry>) -> Result<(), Box<dyn std::error::Error>> {
    let store = app.store("r_store.json")?;

    if store.get("task.countdown").is_none() {
        store.set("task.countdown", 7);
    }

    if store.get("task.clickMode").is_none() {
        store.set("task.clickMode", 1);
    }

    if store.get("task.custom.taskParams").is_none() {
        store.set(
            "task.custom.taskParams",
            json!({ "baseInterval": 150, "maxInterval": 500, "step": 200, "stepInterval": 20, "randomInterval": 25, "xOffset": 20, "yOffset": 20 }),
        );
    }

    if store.get("task.idleMove").is_none() {
        store.set("task.idleMove", 0);
    }

    store.save()?;

    Ok(())
}
