use anyhow::Result;
use pyframe_macros::pyframe_event_api;

use crate::api_manager::ApiManager;

pub fn register_api_instances(api_manager: &mut ApiManager) {
    api_manager.register_event_api("shortcut.register", register);
    api_manager.register_event_api("shortcut.unregister", unregister);
    api_manager.register_event_api("shortcut.unregisterAll", unregister_all);
    api_manager.register_event_api("shortcut.list", list);
}

#[pyframe_event_api]
fn register(shortcut: crate::options::FrameShortcutOption, window_id: Option<u8>) -> Result<u8> {
    app.shortcut()?.register(
        window_id.unwrap_or(window.id),
        shortcut.modifier,
        shortcut.key,
        shortcut.accelerator_str,
    )
}

#[pyframe_event_api]
fn unregister(id: u8, window_id: Option<u8>) -> Result<()> {
    app.shortcut()?.unregister(window_id.unwrap_or(window.id), id)
}

#[pyframe_event_api]
fn unregister_all(window_id: Option<u8>) -> Result<()> {
    app.shortcut()?.unregister_all(window_id.unwrap_or(window.id))
}

#[pyframe_event_api]
fn list(window_id: Option<u8>) -> Result<Vec<(u8, String)>> {
    app.shortcut()?.list(window_id.unwrap_or(window.id))
}
