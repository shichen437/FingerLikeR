use crate::{
    core::mouse::model::{click_params::ClickParams, click_task::TaskStatus},
    infra::store::get_i64,
    shared::app_state::AppState,
};
use enigo::{Button, Coordinate, Direction, Enigo, Mouse, Settings};
use rand::Rng;
use std::{thread, time::Duration};
use tauri::{AppHandle, Wry};
use tauri_plugin_store::StoreExt;

pub fn start_clicking(
    app: AppHandle<Wry>,
    window: tauri::WebviewWindow,
    count: u32,
    state: AppState,
) {
    thread::spawn(move || {
        run_task(&app, &window, count, &state);
        state.finish_task();
    });
}

fn run_task(app: &AppHandle<Wry>, window: &tauri::WebviewWindow, count: u32, state: &AppState) {
    if !state.is_running() {
        state.set_status(window, TaskStatus::Cancelled);
        return;
    }
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let (x, y) = get_location();

    let click_mode = get_i64("task.clickMode", 1);

    match click_mode {
        2 => execute_fixed_click_loop(&mut enigo, window, count, x, y, state),
        3 => {
            let params = get_custom_params(app);
            execute_random_click_loop(&mut enigo, window, count, x, y, &params, state);
        }
        _ => {
            let params = ClickParams::default();
            execute_random_click_loop(&mut enigo, window, count, x, y, &params, state);
        }
    }
    if state.is_running() {
        state.set_status(window, TaskStatus::Finished);
    }
}

pub fn get_location() -> (i32, i32) {
    let enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.location().unwrap()
}

fn get_custom_params(app: &AppHandle<Wry>) -> ClickParams {
    if let Ok(s) = app.store("r_store.json") {
        if let Some(params) = s.get("task.custom.taskParams") {
            return serde_json::from_value(params.clone()).unwrap_or_default();
        }
    }
    ClickParams::default()
}

fn execute_random_click_loop(
    enigo: &mut Enigo,
    window: &tauri::WebviewWindow,
    count: u32,
    x: i32,
    y: i32,
    params: &ClickParams,
    state: &AppState,
) {
    let mut rng = rand::thread_rng();

    for i in 0..count {
        if !state.is_running() {
            state.set_status(window, TaskStatus::Cancelled);
            return;
        }
        state.set_progress(window, TaskStatus::Running, i + 1, count);
        let x_offset = if params.x_offset_max > 0 {
            rng.gen_range(-params.x_offset_max..=params.x_offset_max)
        } else {
            0
        };
        let y_offset = if params.y_offset_max > 0 {
            rng.gen_range(-params.y_offset_max..=params.y_offset_max)
        } else {
            0
        };
        enigo
            .move_mouse(x + x_offset, y + y_offset, Coordinate::Abs)
            .unwrap();

        enigo.button(Button::Left, Direction::Click).unwrap();

        let current_click_index = i as f64;
        let mut delay = params.base_interval as f64;

        if params.step >= 100 && params.step_interval > 0 {
            delay +=
                (current_click_index / params.step as f64).floor() * params.step_interval as f64;
        }

        if params.base_interval < params.max_interval {
            delay = delay.min(params.max_interval as f64);
        }

        if params.random_interval > 0 {
            let random_offset = rng.gen_range(-params.random_interval..=params.random_interval);
            delay += random_offset as f64;
        }

        if delay < 20.0 {
            delay = 20.0;
        }

        thread::sleep(Duration::from_millis(delay as u64));
    }
}

fn execute_fixed_click_loop(
    enigo: &mut Enigo,
    window: &tauri::WebviewWindow,
    count: u32,
    x: i32,
    y: i32,
    state: &AppState,
) {
    enigo.move_mouse(x, y, Coordinate::Abs).unwrap();
    for i in 0..count {
        if !state.is_running() {
            state.set_status(window, TaskStatus::Cancelled);
            return;
        }
        state.set_progress(window, TaskStatus::Running, i + 1, count);
        enigo.button(Button::Left, Direction::Click).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
}
