// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::{
    lock, try_or_log_err,
    utils::{get_json_sync, FrameEvent, FrameWindowTarget, UserEvent},
    CoreApplication,
};
use anyhow::Result;
use serde_json::json;
use std::sync::Arc;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopWindowTarget},
    window::WindowId,
};

pub struct EventHandler {
    app: Arc<CoreApplication>,
    active_window_id: std::sync::Mutex<Option<WindowId>>,
    _window_id: WindowId,
    tray_icon: Option<tray_icon::TrayIcon>,
}

impl EventHandler {
    pub fn new(app: Arc<CoreApplication>, _window_id: WindowId) -> Self {
        Self {
            app,
            active_window_id: std::sync::Mutex::new(None),
            _window_id,
            tray_icon: None,
        }
    }

    pub fn handle(&mut self, event: Event<UserEvent>, target: &FrameWindowTarget, control_flow: &mut ControlFlow) {
        try_or_log_err!({
            *control_flow = ControlFlow::Wait;
            match event {
                Event::NewEvents(tao::event::StartCause::Init) => {
                    println!("PyFrame Startet Up");
                    if self.tray_icon.is_none() {
                        let tray = self.create_tray_icon()?;
                        self.tray_icon = tray;
                    }
                }
                Event::WindowEvent { event, window_id, .. } => {
                    self.handle_window_event(event, window_id, control_flow)?
                }
                Event::UserEvent(user_event) => match user_event {
                    UserEvent::FrameEvent(callback) => self.handle_user_event(callback, target, control_flow)?,
                    UserEvent::MenuEvent(_menu_event) => {
                        self.handle_menu_event(_menu_event)?;
                    }
                },

                _ => (),
            }

            Ok(())
        });
    }

    fn handle_window_event(
        &mut self,
        event: WindowEvent,
        window_id: WindowId,
        control_flow: &mut ControlFlow,
    ) -> Result<()> {
        if event == WindowEvent::Destroyed {
            self.app.window()?.close_window_inner(window_id)?;
        }
        *self.active_window_id.lock().unwrap() = Some(window_id);
        let window = self.app.window()?.get_window_inner(window_id)?;

        match event {
            WindowEvent::Focused(focused) => {
                #[cfg(target_os = "macos")]
                window.switch_menu();
                window.send_ipc_event("window.focused", focused)?;
            }
            WindowEvent::ScaleFactorChanged {
                scale_factor,
                new_inner_size,
            } => {
                window.send_ipc_event(
                    "window.scaleFactorChanged",
                    json!({
                        "scaleFactor": scale_factor,
                        "newInnerSize": new_inner_size
                    }),
                )?;
            }
            WindowEvent::ThemeChanged(theme) => {
                window.send_ipc_event(
                    "window.themeChanged",
                    match theme {
                        tao::window::Theme::Dark => "dark",
                        tao::window::Theme::Light => "light",
                        _ => "system",
                    },
                )?;
            }
            WindowEvent::CloseRequested => {
                let is_block_closed_requested = { lock!(window.state)?.is_block_closed_requested };
                if is_block_closed_requested {
                    window.send_ipc_event("window.closeRequested", json!(null))?;
                } else {
                    self.app.window()?.close_window_inner(window_id)?;
                    if window.id == 0 {
                        let host = self.app.launch_info.options.host.clone();
                        let port = self.app.launch_info.options.port;
                        let server_url = format!("http://{}:{}/server_shutdown", host, port);

                        match get_json_sync(&server_url) {
                            Ok(json) => {
                                if json["status"] == 200 {
                                    if let Some(tray) = self.tray_icon.take() {
                                        drop(tray); // Explizit "destroy"
                                    }
                                    *control_flow = ControlFlow::Exit;
                                } else {
                                    eprintln!("Shutdown-Status: {:?}", json);
                                }
                            }
                            Err(err) => {
                                eprintln!("Fehler beim Server-Shutdown: {}", err);
                            }
                        }
                    }
                }
            }
            _ => (),
        }
        Ok(())
    }

    fn handle_menu_event(&self, menu_event: muda::MenuEvent) -> Result<()> {
        let binding = self.app.window()?;

        let menu_api = self.app.menu()?;

        let items = &menu_api.items;

        let window_id: WindowId = self.active_window_id.lock().unwrap().unwrap();
        let window = binding.get_window_inner(window_id)?;

        if let Some((kind, function_info)) = items.get(menu_event.id()) {
            match kind {
                muda::MenuItemKind::MenuItem(item) => {
                    let item_id = item.id().0.clone();
                    if let Some(func_info) = function_info {
                        let payload = serde_json::json!({
                            "protocol": "menu",
                            "payload":{
                                "command_id":func_info,
                                "extra_args": [item_id],
                                "extra_kwargs":{}
                            }
                        });

                        // window.send_ipc_event("window.menu_comand_handel", json!(payload))?;
                        window.post_message(payload)?;
                    }
                }
                muda::MenuItemKind::Submenu(sub) => {
                    if let Some(func_info) = function_info {
                        let payload = serde_json::json!({
                            "event": "menu",
                            "kind":"submenu",
                            "command_id": func_info,
                            "is_enabled":sub.is_enabled()
                        });
                        // window.send_ipc_event("window.menu_comand_handel", json!(payload))?;
                        window.post_message(payload)?;
                    }
                }
                muda::MenuItemKind::Predefined(pre) => {
                    if let Some(func_info) = function_info {
                        let payload = serde_json::json!({
                            "event": "menu",
                            "kind":"predefined",
                            "command_id": func_info,
                            "text":pre.text()
                        });
                        // window.send_ipc_event("window.menu_comand_handel", json!(payload))?;
                        window.post_message(payload)?;
                    }
                }
                muda::MenuItemKind::Check(check) => {
                    if let Some(func_info) = function_info {
                        let payload = serde_json::json!({
                            "event": "menu",
                            "kind":"check",
                            "command_id": func_info,
                            "checked":check.is_checked()
                        });
                        // window.send_ipc_event("window.menu_comand_handel", json!(payload))?;
                        window.post_message(payload)?;
                    }
                }
                muda::MenuItemKind::Icon(_) => {
                    if let Some(func_info) = function_info {
                        println!("Icon command_id: {:?}", func_info);
                    }
                }
            }
        }

        Ok(())
    }

    fn handle_user_event(
        &self,
        callback: FrameEvent,
        target: &EventLoopWindowTarget<UserEvent>,
        control_flow: &mut ControlFlow,
    ) -> Result<()> {
        callback(target, control_flow)
    }

    fn create_tray_icon(&self) -> Result<Option<tray_icon::TrayIcon>> {
        let menu = self.app.menu()?;
        let tray_icon_options = self
            .app
            .launch_info
            .options
            .window_menu
            .clone()
            .unwrap()
            .system_tray
            .clone();

        let menu_mode = self.app.launch_info.options.menu_mode.clone();

        let mut tray_icon = None;

        match menu_mode {
            Some(crate::options::MenuMode::Menu) => {
                // Nur Menü in Fenster – kein TrayIcon
            }
            Some(crate::options::MenuMode::Tray) => {
                // Tray-Icon mit demselben Menü wie das Fenster
                let window_menu = menu.get_menu_manager()?; // Beispielmethode, die das Fenster-Menü zurückgib

                if let Some(tra_options) = &tray_icon_options {
                    tray_icon = Some(crate::hylper::init_sys_tray(tra_options.clone(), window_menu)?);
                }
            }
            Some(crate::options::MenuMode::MenuAndTray) => {
                // Tray-Icon mit demselben Menü wie das Fenster
                let window_menu = menu.get_menu_manager()?; // Beispielmethode, die das Fenster-Menü zurückgib

                if let Some(tra_options) = &tray_icon_options {
                    tray_icon = Some(crate::hylper::init_sys_tray(tra_options.clone(), window_menu)?);
                }
            }
            None => {
                // Gar nichts tun
            }
        }

        #[cfg(target_os = "macos")]
        {
            use objc2_core_foundation::CFRunLoop;
            let rl = CFRunLoop::main().unwrap();
            rl.wake_up();
        }

        Ok(tray_icon)
    }
}
