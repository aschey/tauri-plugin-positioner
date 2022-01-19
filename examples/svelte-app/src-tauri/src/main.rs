#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use tauri::{Manager, SystemTray, SystemTrayEvent};
use tauri_plugin_positioner::{on_tray_event, Position, Positioner, WindowExt};

fn main() {
  tauri::Builder::default()
    .plugin(Positioner::default())
    .setup(|app| {
      let win = app.get_window("main").unwrap();

      tauri::async_runtime::spawn(async move {
        let _ = win.move_window(Position::TopRight);
      });

      Ok(())
    })
    .system_tray(SystemTray::new())
    .on_system_tray_event(|app, event| {
      on_tray_event(app, &event);
      if let SystemTrayEvent::LeftClick { .. } = event {
        let win = app.get_window("main").unwrap();
        let _ = win.move_window(Position::TrayCenter);
      }
    })
    .run(tauri::generate_context!())
    .expect("failed to run app");
}
