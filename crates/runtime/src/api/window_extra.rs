// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::api_manager::ApiManager;
#[cfg(any(target_os = "windows", target_os = "macos"))]
use anyhow::Result;

#[cfg(any(target_os = "windows", target_os = "macos"))]
use pyframe_macros::pyframe_api;

#[cfg(target_os = "macos")]
use tao::platform::macos::WindowExtMacOS;
#[cfg(target_os = "windows")]
use tao::platform::windows::WindowExtWindows;
#[cfg(target_os = "windows")]
use tao::window::Theme;

#[cfg(any(target_os = "windows", target_os = "macos"))]
macro_rules! match_window {
    ($app:ident, $window:ident, $id:ident) => {
        let $window = match $id {
            Some(id) => $app.window()?.get_window(id)?,
            None => $window,
        };
    };
}

pub fn register_api_instances(_api_manager: &mut ApiManager) {
    #[cfg(target_os = "windows")]
    {
        _api_manager.register_api("windowExtra.setEnable", set_enable);
        _api_manager.register_api("windowExtra.setTaskbarIcon", set_taskbar_icon);
        _api_manager.register_api("windowExtra.theme", theme);
        _api_manager.register_api("windowExtra.resetDeadKeys", reset_dead_keys);
        _api_manager.register_api("windowExtra.beginResizeDrag", begin_resize_drag);
        _api_manager.register_api("windowExtra.setSkipTaskbar", set_skip_taskbar);
        _api_manager.register_api("windowExtra.setUndecoratedShadow", set_undecorated_shadow);
    }

    #[cfg(target_os = "macos")]
    {
        _api_manager.register_api("windowExtra.simpleFullscreen", simple_fullscreen);
        _api_manager.register_api("windowExtra.setSimpleFullscreen", set_simple_fullscreen);
        _api_manager.register_api("windowExtra.hasShadow", has_shadow);
        _api_manager.register_api("windowExtra.setHasShadow", set_has_shadow);
        _api_manager.register_api("windowExtra.setIsDocumentEdited", set_is_document_edited);
        _api_manager.register_api("windowExtra.isDocumentEdited", is_document_edited);
        _api_manager.register_api(
            "windowExtra.setAllowsAutomaticWindowTabbing",
            set_allows_automatic_window_tabbing,
        );
        _api_manager.register_api(
            "windowExtra.allowsAutomaticWindowTabbing",
            allows_automatic_window_tabbing,
        );
        _api_manager.register_api("windowExtra.setTabbingIdentifier", set_tabbing_identifier);
        _api_manager.register_api("windowExtra.tabbingIdentifier", tabbing_identifier);
    }
}

// windows
#[cfg(target_os = "windows")]
#[pyframe_api]
fn set_enable(enabled: bool, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_enable(enabled);
    Ok(())
}

#[cfg(target_os = "windows")]
#[pyframe_api]
fn set_taskbar_icon(taskbar_icon: String, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    let taskbar_icon = app.resource().load_icon(&taskbar_icon)?;
    window.set_taskbar_icon(Some(taskbar_icon));
    Ok(())
}

#[cfg(target_os = "windows")]
#[pyframe_api]
fn theme(id: Option<u8>) -> Result<String> {
    match_window!(app, window, id);
    match window.theme() {
        Theme::Dark => Ok("dark".to_string()),
        Theme::Light => Ok("light".to_string()),
        _ => Ok("system".to_string()),
    }
}

#[cfg(target_os = "windows")]
#[pyframe_api]
fn reset_dead_keys(id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.reset_dead_keys();
    Ok(())
}

#[cfg(target_os = "windows")]
#[pyframe_api]
fn begin_resize_drag(edge: isize, button: u32, x: i32, y: i32, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.begin_resize_drag(edge, button, x, y);
    Ok(())
}

#[cfg(target_os = "windows")]
#[pyframe_api]
fn set_skip_taskbar(skip: bool, id: Option<u8>) -> anyhow::Result<()> {
    match_window!(app, window, id);
    window.set_skip_taskbar(skip)?;
    Ok(())
}

#[cfg(target_os = "windows")]
#[pyframe_api]
fn set_undecorated_shadow(shadow: bool, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_undecorated_shadow(shadow);
    Ok(())
}

#[cfg(target_os = "macos")]
#[pyframe_api]
fn simple_fullscreen(id: Option<u8>) -> Result<bool> {
    match_window!(app, window, id);
    Ok(window.simple_fullscreen())
}

#[cfg(target_os = "macos")]
#[pyframe_api]
fn set_simple_fullscreen(fullscreen: bool, id: Option<u8>) -> Result<bool> {
    match_window!(app, window, id);
    Ok(window.set_simple_fullscreen(fullscreen))
}

#[pyframe_api]
#[cfg(target_os = "macos")]
fn has_shadow(id: Option<u8>) -> Result<bool> {
    match_window!(app, window, id);
    Ok(window.has_shadow())
}

#[cfg(target_os = "macos")]
#[pyframe_api]
fn set_has_shadow(has_shadow: bool, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_has_shadow(has_shadow);
    Ok(())
}

#[cfg(target_os = "macos")]
#[pyframe_api]
fn set_is_document_edited(edited: bool, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_is_document_edited(edited);
    Ok(())
}

#[cfg(target_os = "macos")]
#[pyframe_api]
fn is_document_edited(id: Option<u8>) -> Result<bool> {
    match_window!(app, window, id);
    Ok(window.is_document_edited())
}

#[cfg(target_os = "macos")]
#[pyframe_api]
fn set_allows_automatic_window_tabbing(enabled: bool, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_allows_automatic_window_tabbing(enabled);
    Ok(())
}

#[cfg(target_os = "macos")]
#[pyframe_api]
fn allows_automatic_window_tabbing(id: Option<u8>) -> Result<bool> {
    match_window!(app, window, id);
    Ok(window.allows_automatic_window_tabbing())
}

#[cfg(target_os = "macos")]
#[pyframe_api]
fn set_tabbing_identifier(identifier: String, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_tabbing_identifier(&identifier);
    Ok(())
}

#[cfg(target_os = "macos")]
#[pyframe_api]
fn tabbing_identifier(id: Option<u8>) -> Result<String> {
    match_window!(app, window, id);
    Ok(window.tabbing_identifier())
}
