// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use anyhow::{anyhow, Result};
use pyframe_macros::{pyframe_api, pyframe_event_api};
use serde_json::{json, Value};

use tao::{
    event_loop::ControlFlow,
    window::{CursorIcon, Fullscreen, Theme, UserAttentionType},
};

use crate::{
    api_manager::ApiManager,
    lock, logical, logical_try,
    options::window::{Position, Size, WindowConfig},
};

macro_rules! match_window {
    ($app:ident, $window:ident, $id:ident) => {
        let $window = match $id {
            Some(id) => $app.window()?.get_window(id)?,
            None => $window,
        };
    };
}

pub fn register_api_instances(_api_manager: &mut ApiManager) {
    _api_manager.register_api("window.current", current);
    _api_manager.register_event_api("window.open", open);
    _api_manager.register_event_api("window.close", close);
    _api_manager.register_api("window.list", list);
    _api_manager.register_api("window.sendMessage", send_message);
    /*     _api_manager.register_api("window.setMenu", set_menu);
    _api_manager.register_api("window.hideMenu", hide_menu);
    _api_manager.register_api("window.showMenu", show_menu);
    _api_manager.register_api("window.isMenuVisible", is_menu_visible); */
    _api_manager.register_api("window.scaleFactor", scale_factor);
    _api_manager.register_api("window.innerPosition", inner_position);
    _api_manager.register_api("window.outerPosition", outer_position);
    _api_manager.register_api("window.setOuterPosition", set_outer_position);
    _api_manager.register_api("window.innerSize", inner_size);
    _api_manager.register_api("window.setInnerSize", set_inner_size);
    _api_manager.register_api("window.outerSize", outer_size);
    _api_manager.register_api("window.setMinInnerSize", set_min_inner_size);
    _api_manager.register_api("window.setMaxInnerSize", set_max_inner_size);
    _api_manager.register_api("window.setTitle", set_title);
    _api_manager.register_api("window.title", title);
    _api_manager.register_api("window.isVisible", is_visible);
    _api_manager.register_api("window.setVisible", set_visible);
    _api_manager.register_api("window.isFocused", is_focused);
    _api_manager.register_api("window.setFocus", set_focus);
    _api_manager.register_api("window.isResizable", is_resizable);
    _api_manager.register_api("window.setResizable", set_resizable);
    _api_manager.register_api("window.isMinimizable", is_minimizable);
    _api_manager.register_api("window.setMinimizable", set_minimizable);
    _api_manager.register_api("window.isMaximizable", is_maximizable);
    _api_manager.register_api("window.setMaximizable", set_maximizable);
    _api_manager.register_api("window.isClosable", is_closable);
    _api_manager.register_api("window.setClosable", set_closable);
    _api_manager.register_api("window.isMinimized", is_minimized);
    _api_manager.register_api("window.setMinimized", set_minimized);
    _api_manager.register_api("window.isMaximized", is_maximized);
    _api_manager.register_api("window.setMaximized", set_maximized);
    _api_manager.register_api("window.Decorated", decorated);
    _api_manager.register_api("window.setDecorated", set_decorated);
    _api_manager.register_api("window.fullscreen", fullscreen);
    _api_manager.register_api("window.setFullscreen", set_fullscreen);
    _api_manager.register_api("window.setAlwaysOnTop", set_always_on_top);
    _api_manager.register_api("window.setAlwaysOnBottom", set_always_on_bottom);
    _api_manager.register_api("window.requestUserAttention", request_user_attention);
    _api_manager.register_api("window.setContentProtection", set_content_protection);
    _api_manager.register_api("window.setVisibleOnAllWorkspaces", set_visible_on_all_workspaces);
    _api_manager.register_api("window.setCursorIcon", set_cursor_icon);
    _api_manager.register_api("window.cursorPosition", cursor_position);
    _api_manager.register_api("window.setCursorPosition", set_cursor_position);
    _api_manager.register_api("window.setCursorGrab", set_cursor_grab);
    _api_manager.register_api("window.setCursorVisible", set_cursor_visible);
    _api_manager.register_api("window.dragWindow", drag_window);
    _api_manager.register_api("window.setIgnoreCursorEvents", set_ignore_cursor_events);
    _api_manager.register_api("window.theme", theme);
    _api_manager.register_api("window.blockCloseRequested", block_close_requested);
}

#[pyframe_api]
fn current() -> Result<u8> {
    Ok(window.id)
}

#[pyframe_event_api]
fn open(options: Option<WindowConfig>) -> Result<u8> {
    let new_window = app.window()?.open_window(target, &options.unwrap_or_default())?;
    Ok(new_window.id)
}

#[pyframe_event_api]
fn close(id: Option<u8>) -> Result<()> {
    let id = id.unwrap_or(window.id);
    if id == 0 {
        *control_flow = ControlFlow::Exit;
        return Ok(());
    }
    app.window()?.close_window(id)
}

#[pyframe_api]
fn list() -> Result<Vec<Value>> {
    Ok(app
        .window()?
        .list_windows()
        .into_iter()
        .map(|w| json!({"id":w.id,"title":w.title(),"visible":w.is_visible(),}))
        .collect())
}

#[pyframe_api]
fn send_message(message: String, id: u8) -> Result<()> {
    let remote = app.window()?.get_window(id)?;
    remote.send_ipc_event("window.message", json!({"from":window.id,"message":message,}))?;
    Ok(())
}

/* #[pyframe_api]
fn set_menu(options: Option<WindowMenuOptions>, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_menu(&options);
    Ok(())
}

#[pyframe_api]
fn hide_menu(id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.hide_menu();
    Ok(())
}

#[pyframe_api]
fn show_menu(id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.hide_menu();
    Ok(())
}

#[pyframe_api]
fn is_menu_visible(id: Option<u8>) -> Result<bool> {
    match_window!(app, window, id);
    Ok(window.is_menu_visible())
}
 */
#[pyframe_api]
fn scale_factor(id: Option<u8>) -> Result<f64> {
    match_window!(app, window, id);
    Ok(window.scale_factor())
}

#[pyframe_api]
fn inner_position(id: Option<u8>) -> Result<Position> {
    match_window!(app, window, id);
    Ok(logical_try!(window, inner_position))
}

#[pyframe_api]
fn outer_position(id: Option<u8>) -> Result<Position> {
    match_window!(app, window, id);
    Ok(logical_try!(window, outer_position))
}

#[pyframe_api]
fn set_outer_position(position: Position, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_outer_position(position);
    Ok(())
}

#[pyframe_api]
fn inner_size(id: Option<u8>) -> Result<Size> {
    match_window!(app, window, id);
    Ok(logical!(window, inner_size))
}

#[pyframe_api]
fn set_inner_size(size: Size, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_inner_size(size);
    Ok(())
}

#[pyframe_api]
fn outer_size(id: Option<u8>) -> Result<Size> {
    match_window!(app, window, id);
    Ok(logical!(window, outer_size))
}

#[pyframe_api]
fn set_min_inner_size(size: Size, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_min_inner_size(Some(size));
    Ok(())
}

#[pyframe_api]
fn set_max_inner_size(size: Size, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_max_inner_size(Some(size));
    Ok(())
}

#[pyframe_api]
fn set_title(title: String, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_title(&title);
    Ok(())
}

#[pyframe_api]
fn title(id: Option<u8>) -> Result<String> {
    match_window!(app, window, id);
    Ok(window.title())
}

#[pyframe_api]
fn is_visible(id: Option<u8>) -> Result<bool> {
    match_window!(app, window, id);
    Ok(window.is_visible())
}

#[pyframe_api]
fn set_visible(visible: bool, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_visible(visible);
    Ok(())
}

#[pyframe_api]
fn is_focused(id: Option<u8>) -> Result<bool> {
    match_window!(app, window, id);
    Ok(window.is_focused())
}

#[pyframe_api]
fn set_focus(id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_focus();
    Ok(())
}

#[pyframe_api]
fn is_resizable(id: Option<u8>) -> Result<bool> {
    match_window!(app, window, id);
    Ok(window.is_resizable())
}

#[pyframe_api]
fn set_resizable(resizable: bool, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_resizable(resizable);
    Ok(())
}

#[pyframe_api]
fn is_minimizable(id: Option<u8>) -> Result<bool> {
    match_window!(app, window, id);
    Ok(window.is_minimizable())
}

#[pyframe_api]
fn set_minimizable(minimizable: bool, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_minimizable(minimizable);
    Ok(())
}

#[pyframe_api]
fn is_maximizable(id: Option<u8>) -> Result<bool> {
    match_window!(app, window, id);
    Ok(window.is_maximizable())
}

#[pyframe_api]
fn set_maximizable(maximizable: bool, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_maximizable(maximizable);
    Ok(())
}

#[pyframe_api]
fn is_closable(id: Option<u8>) -> Result<bool> {
    match_window!(app, window, id);
    Ok(window.is_closable())
}

#[pyframe_api]
fn set_closable(closable: bool, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_closable(closable);
    Ok(())
}

#[pyframe_api]
fn is_minimized(id: Option<u8>) -> Result<bool> {
    match_window!(app, window, id);
    Ok(window.is_minimized())
}

#[pyframe_api]
fn set_minimized(minimized: bool, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_minimized(minimized);
    Ok(())
}

#[pyframe_api]
fn is_maximized(id: Option<u8>) -> Result<bool> {
    match_window!(app, window, id);
    Ok(window.is_maximized())
}

#[pyframe_api]
fn set_maximized(maximized: bool, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_maximized(maximized);
    Ok(())
}

#[pyframe_api]
fn decorated(id: Option<u8>) -> Result<bool> {
    match_window!(app, window, id);
    Ok(window.is_decorated())
}

#[pyframe_api]
fn set_decorated(decorated: bool, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_decorations(decorated);
    Ok(())
}

#[pyframe_api]
fn fullscreen(id: Option<u8>) -> Result<bool> {
    match_window!(app, window, id);
    Ok(window.fullscreen().is_some())
}

#[pyframe_api]
fn set_fullscreen(is_fullscreen: bool, monitor_name: Option<String>, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    if !is_fullscreen {
        window.set_fullscreen(None);
        return Ok(());
    }
    match monitor_name {
        Some(name) => {
            let monitor = window.available_monitors().find(|m| m.name() == Some(name.clone()));
            if monitor.is_none() {
                return Err(anyhow!("Monitornotfound"));
            }
            window.set_fullscreen(Some(Fullscreen::Borderless(monitor)));
        }
        None => {
            window.set_fullscreen(Some(Fullscreen::Borderless(None)));
        }
    };
    Ok(())
}

#[pyframe_api]
fn set_always_on_top(always_on_top: bool, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_always_on_top(always_on_top);
    Ok(())
}

#[pyframe_api]
fn set_always_on_bottom(always_on_bottom: bool, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_always_on_bottom(always_on_bottom);
    Ok(())
}

#[pyframe_api]
fn request_user_attention(level: String, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    match level.as_str() {
        "informational" => window.request_user_attention(Some(UserAttentionType::Informational)),
        "critical" => window.request_user_attention(Some(UserAttentionType::Critical)),
        _ => window.request_user_attention(None),
    }
    Ok(())
}

#[pyframe_api]
fn set_content_protection(enabled: bool, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_content_protection(enabled);
    Ok(())
}

#[pyframe_api]
fn set_visible_on_all_workspaces(visible: bool, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_visible_on_all_workspaces(visible);
    Ok(())
}

#[pyframe_api]
fn set_cursor_icon(icon: String, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_cursor_icon(match icon.as_str() {
        "default" => CursorIcon::Default,
        "crosshair" => CursorIcon::Crosshair,
        "hand" => CursorIcon::Hand,
        "arrow" => CursorIcon::Arrow,
        "move" => CursorIcon::Move,
        "text" => CursorIcon::Text,
        "wait" => CursorIcon::Wait,
        "help" => CursorIcon::Help,
        "progress" => CursorIcon::Progress,
        "not_allowed" => CursorIcon::NotAllowed,
        "context_menu" => CursorIcon::ContextMenu,
        "cell" => CursorIcon::Cell,
        "vertical_text" => CursorIcon::VerticalText,
        "alias" => CursorIcon::Alias,
        "copy" => CursorIcon::Copy,
        "no_drop" => CursorIcon::NoDrop,
        "grab" => CursorIcon::Grab,
        "grabbing" => CursorIcon::Grabbing,
        "all_scroll" => CursorIcon::AllScroll,
        "zoom_in" => CursorIcon::ZoomIn,
        "zoom_out" => CursorIcon::ZoomOut,
        "e_resize" => CursorIcon::EResize,
        "n_resize" => CursorIcon::NResize,
        "ne_resize" => CursorIcon::NeResize,
        "nw_resize" => CursorIcon::NwResize,
        "s_resize" => CursorIcon::SResize,
        "se_resize" => CursorIcon::SeResize,
        "sw_resize" => CursorIcon::SwResize,
        "w_resize" => CursorIcon::WResize,
        "ew_resize" => CursorIcon::EwResize,
        "ns_resize" => CursorIcon::NsResize,
        "nesw_resize" => CursorIcon::NeswResize,
        "nwse_resize" => CursorIcon::NwseResize,
        "col_resize" => CursorIcon::ColResize,
        "row_resize" => CursorIcon::RowResize,
        _ => CursorIcon::Arrow,
    });
    Ok(())
}

#[pyframe_api]
fn cursor_position(id: Option<u8>) -> Result<Position> {
    match_window!(app, window, id);
    Ok(logical_try!(window, cursor_position))
}

#[pyframe_api]
fn set_cursor_position(position: Position, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_cursor_position(position)?;
    Ok(())
}

#[pyframe_api]
fn set_cursor_grab(grab: bool, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_cursor_grab(grab)?;
    Ok(())
}

#[pyframe_api]
fn set_cursor_visible(visible: bool, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_cursor_visible(visible);
    Ok(())
}

#[pyframe_api]
fn drag_window(id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.drag_window()?;
    Ok(())
}

#[pyframe_api]
fn set_ignore_cursor_events(ignore: bool, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    window.set_ignore_cursor_events(ignore)?;
    Ok(())
}

#[pyframe_api]
fn theme(id: Option<u8>) -> Result<String> {
    match_window!(app, window, id);
    Ok(String::from(match window.theme() {
        Theme::Light => "light",
        Theme::Dark => "dark",
        _ => "system",
    }))
}

#[pyframe_api]
fn block_close_requested(blocked: bool, id: Option<u8>) -> Result<()> {
    match_window!(app, window, id);
    let mut state = lock!(window.state)?;
    state.is_block_closed_requested = blocked;
    Ok(())
}
