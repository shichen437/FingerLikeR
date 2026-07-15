use std::sync::{Mutex, OnceLock};
use tauri::{AppHandle, Wry};

static APP_HANDLE: OnceLock<Mutex<AppHandle<Wry>>> = OnceLock::new();

pub fn set_app_handle(handle: AppHandle<Wry>) {
    APP_HANDLE
        .set(Mutex::new(handle))
        .expect("APP_HANDLE already set");
}

pub fn app_handle() -> AppHandle<Wry> {
    let lock = APP_HANDLE
        .get()
        .expect("APP_HANDLE not initialized, call set_app_handle first")
        .lock()
        .expect("APP_HANDLE poisoned");

    lock.clone()
}
