use crate::{
    application::state::AppState,
    domain::models::{click_params::ClickParams, task::TaskStatus},
};
use enigo::{Button, Coordinate, Direction, Enigo, Mouse, Settings};
use rand::Rng;
use std::{thread, time::Duration};
use tauri::{AppHandle, Manager, Wry};
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

    let click_mode = app
        .store("r_store.json")
        .ok()
        .and_then(|s| s.get("task.clickMode").and_then(|v| v.as_i64()))
        .unwrap_or(1);

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

pub fn move_mouse_randomly_smooth(app_handle: AppHandle<Wry>) {
    let state = app_handle.state::<AppState>();
    if state.is_running() {
        return;
    }
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let (screen_width, screen_height) = enigo.main_display().unwrap();
    let (current_x, current_y) = enigo.location().unwrap();

    let mut rng = rand::thread_rng();

    let x_offset = rng.gen_range(-150..=150);
    let y_offset = rng.gen_range(-150..=150);

    let mut target_x = current_x + x_offset;
    let mut target_y = current_y + y_offset;

    target_x = target_x.clamp(0, screen_width.saturating_sub(1));
    target_y = target_y.clamp(0, screen_height.saturating_sub(1));

    let steps = 40;
    let step_delay = Duration::from_millis(8);

    let dx = (target_x - current_x) as f32;
    let dy = (target_y - current_y) as f32;

    for i in 1..=steps {
        let t = i as f32 / steps as f32;

        let x = current_x as f32 + dx * t;
        let y = current_y as f32 + dy * t;

        let x_i = x.round() as i32;
        let y_i = y.round() as i32;

        enigo.move_mouse(x_i, y_i, Coordinate::Abs).unwrap();
        thread::sleep(step_delay);
    }
}
