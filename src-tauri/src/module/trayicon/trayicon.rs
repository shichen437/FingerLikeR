use tauri::async_runtime;
use tauri::image::Image;
use tauri::menu::CheckMenuItem;
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri::Manager;

use crate::infra::store::{get_i64, set_i32};
use crate::module::scheduler::scheduler::toggle_idle_move_job;
use crate::shared::app_handle::app_handle;

pub fn init_tray_menu() -> Result<(), Box<dyn std::error::Error>> {
    let app = app_handle();
    let idle_enabled = get_i64("task.idleMove", 0) == 1;
    let home_i = MenuItem::with_id(&app, "home", "仪表板", true, None::<&str>)?;
    let idle_move_i = CheckMenuItem::with_id(
        &app,
        "idle_move",
        "闲动模式",
        true,
        idle_enabled,
        None::<&str>,
    )?;
    let quit_i = MenuItem::with_id(&app, "quit", "退出", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(&app)?;
    let menu = Menu::with_items(
        &app,
        &[&home_i, &separator, &idle_move_i, &separator, &quit_i],
    )?;

    let idle_move_item = idle_move_i.clone();
    let idle_move_item_clone = idle_move_item.clone();
    let app_clone = app.clone();

    TrayIconBuilder::new()
        .icon(
            Image::from_bytes(include_bytes!("../../../icons/tray-icon/tray-icon-mac.png"))
                .expect("REASON"),
        )
        .icon_as_template(true)
        .show_menu_on_left_click(false)
        .menu(&menu)
        .on_tray_icon_event(move |_tray, event| match event {
            TrayIconEvent::Click { button, .. } => {
                if button == tauri::tray::MouseButton::Right {
                    let enabled = get_i64("task.idleMove", 0) == 1;
                    let _ = idle_move_item_clone.set_checked(enabled);
                }

                if button == tauri::tray::MouseButton::Left {
                    if let Some(window) = app_clone.get_webview_window("main") {
                        let _ = window.unminimize();
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
            _ => {}
        })
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "home" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.unminimize();
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "idle_move" => {
                let current = get_i64("task.idleMove", 0);
                let new_enabled = current == 0;

                let _ = set_i32("task.idleMove", if new_enabled { 1 } else { 0 });
                let _ = idle_move_item.set_checked(new_enabled);

                async_runtime::spawn(async move {
                    let _ = toggle_idle_move_job();
                });
            }
            "quit" => {
                app.exit(0);
            }
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
        .build(&app)?;

    Ok(())
}
