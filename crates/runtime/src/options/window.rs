// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use serde::Deserialize;
use std::collections::HashMap;
#[cfg(any(target_os = "windows", target_os = "linux"))]
use std::path::PathBuf;
use tao::dpi::{LogicalPosition, LogicalSize};

use super::menu::MenuFrame;

pub type Size = LogicalSize<f64>;
pub type Position = LogicalPosition<f64>;

#[cfg(target_os = "linux")]
#[derive(Debug, Clone, Deserialize, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinuxWindowConfig {
    pub skip_taskbar: Option<bool>,
    pub transient_for: Option<i32>,
    pub transparent_draw: Option<bool>,
    pub double_buffered: Option<bool>,
    pub rgba_visual: Option<bool>,
    pub app_paintable: Option<bool>,
    pub cursor_moved_event: Option<bool>,
    pub default_vbox: Option<bool>,
    pub extensions_path: Option<PathBuf>,
}

#[cfg(target_os = "windows")]
#[derive(Debug, Clone, Deserialize, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowsWindowConfig {
    pub parent_window: Option<u8>,
    pub owner_window: Option<i32>,
    pub menu: Option<i32>,
    pub taskbar_icon: Option<String>,
    pub no_redirection_bitmap: Option<bool>,
    pub drag_and_drop: Option<bool>,
    pub skip_taskbar: Option<bool>,
    pub window_classname: Option<String>,
    pub undecorated_shadow: Option<bool>,
    pub rtl: Option<bool>,
    pub additional_browser_args: Option<String>,
    pub browser_accelerator_keys: Option<bool>,
    pub default_context_menus: Option<bool>,
    pub theme: Option<String>,
    pub https_scheme: Option<bool>,
    pub scroll_bar_style: Option<String>,
    pub browser_extensions_enabled: Option<bool>,
    pub extensions_path: Option<PathBuf>,
}
#[cfg(target_os = "macos")]
#[derive(Debug, Clone, Deserialize, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MacOSWindowConfig {
    pub activation_policy: Option<crate::options::ActivationPolicy>,
    pub set_dock_visibility: Option<bool>,
    pub activate_ignoring_other_apps: Option<bool>,
    pub parent_window: Option<u8>,
    pub movable_by_window_background: Option<bool>,
    pub titlebar_transparent: Option<bool>,
    pub titlebar_hidden: Option<bool>,
    pub titlebar_buttons_hidden: Option<bool>,
    pub title_hidden: Option<bool>,
    pub fullsize_content_view: Option<bool>,
    pub resize_increments: Option<Size>,
    pub disallow_hidpi: Option<bool>,
    pub has_shadow: Option<bool>,
    pub traffic_light_inset: Option<Position>,
    pub automatic_window_tabbing: Option<bool>,
    pub tabbing_identifier: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowConfig {
    pub entry: Option<String>,
    pub window_inner_size: Option<Size>,
    pub window_min_inner_size: Option<Size>,
    pub window_max_inner_size: Option<Size>,
    pub window_position: Option<Position>,
    pub window_resizable: Option<bool>,
    pub window_minimizable: Option<bool>,
    pub window_maximizable: Option<bool>,
    pub window_closable: Option<bool>,
    pub window_title: Option<String>,
    pub window_fullscreen: Option<bool>,
    pub window_maximized: Option<bool>,
    pub window_visible: Option<bool>,
    pub window_transparent: Option<bool>,
    pub window_inner_size_constraints: Option<Size>,
    pub window_decorations: Option<bool>,
    pub window_always_on_bottom: Option<bool>,
    pub window_always_on_top: Option<bool>,
    pub window_window_icon: Option<String>,
    pub window_theme: Option<String>,
    pub window_focused: Option<bool>,
    pub window_content_protection: Option<bool>,
    pub window_visible_on_all_workspaces: Option<bool>,
    pub window_background_color: Option<(u8, u8, u8, u8)>,
    pub webview_context_id: Option<String>,
    pub webview_id: Option<String>,
    pub webview_transparent: Option<bool>,
    pub webview_background_color: Option<(u8, u8, u8, u8)>,
    pub webview_visible: Option<bool>,
    pub webview_autoplay: Option<bool>,
    pub webview_initialization_scripts: Option<Vec<String>>,
    pub webview_initialization_main_only: Option<Vec<(String, bool)>>,
    pub webview_headers: Option<HashMap<String, String>>,
    pub webview_user_agent: Option<String>,
    pub webview_devtools: Option<bool>,
    pub webview_hotkeys_zoom: Option<bool>,
    pub webview_clipboard: Option<bool>,
    pub webview_incognito: Option<bool>,
    pub webview_focused: Option<bool>,
    pub webview_bounds: Option<(i32, i32, i32, i32)>,
    pub webview_javascript_disabled: Option<bool>,
    pub webview_accept_first_mouse: Option<bool>,
    pub webview_back_forward_navigation_gestures: Option<bool>,
    pub webview_background_throttling: Option<super::FrameBackgroundThrottlingPolicy>,
    pub webview_proxy_config: Option<serde_json::Value>,
    pub webview_initialization_script_for_main_only: Option<(String, bool)>,
    #[cfg(target_os = "linux")]
    #[serde(flatten)]
    pub linux_extra: Option<LinuxWindowConfig>,
    #[cfg(target_os = "windows")]
    #[serde(flatten)]
    pub windows_extra: Option<WindowsWindowConfig>,
    #[cfg(target_os = "macos")]
    #[serde(flatten)]
    pub macos_extra: Option<MacOSWindowConfig>,
    pub window_menu: Option<MenuFrame>,
}
