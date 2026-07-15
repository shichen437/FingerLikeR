use serde_json::json;
use tauri_plugin_store::StoreExt;

use crate::shared::app_handle::app_handle;

const STORE_NAME: &str = "r_store.json";

const TASK_COUNTDOWN: &str = "task.countdown";
const TASK_CLICK_MODE: &str = "task.clickMode";
const TASK_CUSTOM_PARAMS: &str = "task.custom.taskParams";
const TASK_IDLE_MOVE: &str = "task.idleMove";

pub fn init_store() -> Result<(), Box<dyn std::error::Error>> {
    let app = app_handle();
    let store = app.store(STORE_NAME)?;

    if store.get(TASK_COUNTDOWN).is_none() {
        store.set(TASK_COUNTDOWN, 7);
    }

    if store.get(TASK_CLICK_MODE).is_none() {
        store.set(TASK_CLICK_MODE, 1);
    }

    if store.get(TASK_CUSTOM_PARAMS).is_none() {
        store.set(
            TASK_CUSTOM_PARAMS,
            json!({ "baseInterval": 150, "maxInterval": 500, "step": 200, "stepInterval": 20, "randomInterval": 25, "xOffset": 20, "yOffset": 20 }),
        );
    }

    if store.get(TASK_IDLE_MOVE).is_none() {
        store.set(TASK_IDLE_MOVE, 0);
    }

    store.save()?;

    Ok(())
}

pub fn get_i64(key: &str, default: i64) -> i64 {
    let app = app_handle();
    if let Ok(store) = app.store(STORE_NAME) {
        if let Some(value) = store.get(key) {
            if let Some(n) = value.as_i64() {
                return n;
            }
        }
    }
    default
}

#[allow(dead_code)]
pub fn get_i32(key: &str, default: i32) -> i32 {
    let app = app_handle();
    if let Ok(store) = app.store(STORE_NAME) {
        if let Some(value) = store.get(key) {
            if let Some(n) = value.as_i64() {
                if n >= i32::MIN as i64 && n <= i32::MAX as i64 {
                    return n as i32;
                }
            }
        }
    }
    default
}

pub fn set_i32(key: &str, value: i32) -> Result<(), Box<dyn std::error::Error>> {
    let app = app_handle();
    let store = app.store(STORE_NAME)?;
    store.set(key, value);
    store.save()?;
    Ok(())
}
