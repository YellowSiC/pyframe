// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
pub mod api;
pub mod api_manager;
pub mod assets;
pub mod event_handler;
pub mod hylper;
pub mod menu_manager;
pub mod options;
pub mod resource_manager;
pub mod shortcut_manager;
pub mod tray_manager;
pub mod utils;
pub mod window_manager;

use api::register_api_instances;
use api_manager::ApiManager;
use event_handler::EventHandler;
use menu_manager::PyFrameMenuManager;
use resource_manager::{AppResourceManager, FileSystemResource, ResourceManager};
use std::sync::{Arc, MutexGuard};
use utils::{ArcMut, FrameEventLoopProxy};
use window_manager::WindowManager;

pub struct CoreApplication {
    _resource: Arc<dyn ResourceManager>,
    _window_manager: ArcMut<WindowManager>,
    _api_manager: ArcMut<ApiManager>,
    pub launch_info: options::LaunchInfo,
    _shortcut: ArcMut<shortcut_manager::PyFrameShortcutManager>,
    proxy: FrameEventLoopProxy,
    _menu_bar: ArcMut<PyFrameMenuManager>,
}

impl CoreApplication {
    pub fn new(
        event_loop: &mut utils::FrameEventLoop,
        _menu_bar: muda::Menu,
        data: String,
    ) -> anyhow::Result<std::sync::Arc<Self>> {
        let proxy = event_loop.create_proxy();
        let launch_info = options::LaunchInfo::new(data)?;

        let resource_manager: Arc<dyn ResourceManager> = match &launch_info.options.debug_resource {
            Some(dir) => FileSystemResource::new(dir)?,
            None => AppResourceManager::new()?,
        };

        #[cfg(target_os = "macos")]
        if let Some(macos_extra) = &launch_info.options.macos_extra {
            use self::options::ActivationPolicy;
            use tao::platform::macos::{ActivationPolicy as MacOSActivationPolicy, EventLoopExtMacOS};

            if let Some(p) = macos_extra.activation_policy.clone() {
                let policy = match p {
                    ActivationPolicy::Regular => MacOSActivationPolicy::Regular,
                    ActivationPolicy::Accessory => MacOSActivationPolicy::Accessory,
                    ActivationPolicy::Prohibited => MacOSActivationPolicy::Prohibited,
                };
                event_loop.set_activation_policy(policy);
            }

            if let Some(visibility) = macos_extra.set_dock_visibility {
                event_loop.set_dock_visibility(visibility);
            }

            if let Some(ignore) = macos_extra.activate_ignoring_other_apps {
                event_loop.set_activate_ignoring_other_apps(ignore);
            }
        };

        // create api manager and register api instances
        let api_manager = ApiManager::new(&launch_info.options);
        {
            let mut api_manager = lock!(api_manager)?;
            register_api_instances(&mut api_manager);
        }
        let menu_manager = PyFrameMenuManager::new(_menu_bar);
        let window_manager = WindowManager::new(Some(launch_info.data_dir.clone()));

        let shortcut_manager = shortcut_manager::PyFrameShortcutManager::new(event_loop);

        let app = Self {
            _resource: resource_manager,
            _window_manager: window_manager.clone(),
            _api_manager: api_manager.clone(),
            launch_info: launch_info.clone(),
            _shortcut: shortcut_manager.clone(),
            proxy,
            _menu_bar: menu_manager.clone(),
        };

        let application = std::sync::Arc::new(app);

        lock!(window_manager)?.bind_app(application.clone());
        lock!(menu_manager)?.bind_app(application.clone());
        lock!(menu_manager)?.bind_app(application.clone());
        lock!(shortcut_manager)?.bind_app(application.clone());
        Ok(application)
    }

    pub fn resource(self: &Arc<Self>) -> Arc<dyn ResourceManager> {
        self._resource.clone()
    }

    pub fn window(&self) -> anyhow::Result<MutexGuard<'_, WindowManager>> {
        lock!(self._window_manager)
    }

    pub fn menu(&self) -> anyhow::Result<MutexGuard<'_, PyFrameMenuManager>> {
        lock!(self._menu_bar)
    }

    pub fn shortcut(&self) -> anyhow::Result<MutexGuard<'_, shortcut_manager::PyFrameShortcutManager>> {
        lock!(self._shortcut)
    }

    pub fn api(&self) -> anyhow::Result<MutexGuard<'_, ApiManager>> {
        lock!(self._api_manager)
    }

    pub fn run(self: Arc<Self>, event_loop: utils::FrameEventLoop) -> anyhow::Result<()> {
        let app = &self.clone();
        let menu_eventloop_proxy = app.proxy.clone();
        let options: &options::window::WindowConfig = &app.launch_info.options.window.clone();
        let _main_win = app.window()?.open_window(&event_loop, options)?;
        let _win_id = _main_win.id();

        muda::MenuEvent::set_event_handler(Some(move |event| {
            let _ = menu_eventloop_proxy.send_event(utils::UserEvent::MenuEvent(event));
        }));
        let handler = EventHandler::new(app.clone(), _win_id);
        event_loop.run(move |event, target, control_flow| {
            handler.handle(event, target, control_flow);
        });
    }
}
