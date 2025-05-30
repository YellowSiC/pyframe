// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::api_manager::ApiManager;
pub mod audio;
pub mod comand_handel;
pub mod dialog;
pub mod extra;
pub mod fs;
pub mod geolocation;
pub mod http;
pub mod monitor;
pub mod notifications;
pub mod os;
pub mod process;
pub mod resource;
pub mod shortcut;
pub mod webview;
pub mod window;
pub mod window_extra;
pub fn register_api_instances(_api_manager: &mut ApiManager) {
    comand_handel::register_api_instances(_api_manager);
    shortcut::register_api_instances(_api_manager);
    fs::register_api_instances(_api_manager);
    dialog::register_api_instances(_api_manager);
    window::register_api_instances(_api_manager);
    webview::register_api_instances(_api_manager);
    resource::register_api_instances(_api_manager);
    window_extra::register_api_instances(_api_manager);
    monitor::register_api_instances(_api_manager);
    process::register_api_instances(_api_manager);
    os::register_api_instances(_api_manager);
    #[cfg(target_os = "windows")]
    notifications::register_api_instances(_api_manager);
    os::register_api_instances(_api_manager);
    http::register_api_instances(_api_manager);
    audio::register_api_instances(_api_manager);
    geolocation::register_api_instances(_api_manager);
}
