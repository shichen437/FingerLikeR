use crate::{shared::app_handle::app_handle, shared::app_state::AppState};
use enigo::{Coordinate, Enigo, Mouse, Settings};
use rand::Rng;
use std::{thread, time::Duration};
use tauri::Manager;

pub fn move_mouse_randomly_smooth() {
    let ah = app_handle();
    let state = ah.state::<AppState>();
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
