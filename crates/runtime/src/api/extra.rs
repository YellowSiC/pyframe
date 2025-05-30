// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::api_manager::ApiManager;
use anyhow::Result; // Das Makro anyhow! importieren
use pyframe_macros::pyframe_api;
#[cfg(target_os = "macos")]
use pyframe_macros::pyframe_event_api;

pub fn register_api_instances(_api_manager: &mut ApiManager) {
    _api_manager.register_async_api("extra.getActiveWindowId", get_active_window_id);
    _api_manager.register_async_api("extra.focusByWindowId", focus_by_window_id);

    #[cfg(target_os = "macos")]
    {
        _api_manager.register_event_api("extra.hideApplication", hide_application);
        _api_manager.register_event_api("extra.showApplication", show_application);
        _api_manager.register_event_api("extra.hideOtherApplications", hide_other_applications);
        _api_manager.register_event_api("extra.setActivationPolicy", set_activation_policy);
    }
}

#[cfg(target_os = "macos")]
#[pyframe_event_api]
fn hide_application() -> Result<()> {
    use tao::platform::macos::EventLoopWindowTargetExtMacOS;
    // TODO: target-Referenz korrekt übergeben
    target.hide_application();
    Ok(())
}

#[cfg(target_os = "macos")]
#[pyframe_event_api]
fn show_application() -> Result<()> {
    use tao::platform::macos::EventLoopWindowTargetExtMacOS;
    // TODO: target-Referenz korrekt übergeben
    target.show_application();
    Ok(())
}

#[cfg(target_os = "macos")]
#[pyframe_event_api]
fn hide_other_applications() -> Result<()> {
    use tao::platform::macos::EventLoopWindowTargetExtMacOS;
    target.hide_other_applications();
    Ok(())
}

#[cfg(target_os = "macos")]
#[pyframe_event_api]
fn set_activation_policy(policy: crate::options::ActivationPolicy) -> Result<()> {
    use crate::options::ActivationPolicy as PyframeActivationPolicy;
    use tao::platform::macos::ActivationPolicy as TaoActivationPolicy;
    use tao::platform::macos::EventLoopWindowTargetExtMacOS;

    let policy = match policy {
        PyframeActivationPolicy::Regular => TaoActivationPolicy::Regular,
        PyframeActivationPolicy::Accessory => TaoActivationPolicy::Accessory,
        PyframeActivationPolicy::Prohibited => TaoActivationPolicy::Prohibited,
    };
    target.set_activation_policy_at_runtime(policy);
    // TODO: target-Referenz korrekt übergeben
    Ok(())
}

#[cfg(target_os = "macos")]
#[pyframe_api]
fn get_active_window_id() -> Result<Option<String>> {
    use active_win_pos_rs::get_active_window;

    let window = get_active_window();
    match window {
        Ok(win) => Ok(Some(format!("{}_{}", win.process_id, win.window_id))),
        Err(_) => Ok(None),
    }
}

#[cfg(target_os = "macos")]
#[pyframe_api]
fn focus_by_window_id(id_string: String) -> Result<bool> {
    use anyhow::anyhow;
    use objc2::{class, msg_send, runtime::AnyObject};
    use objc2_app_kit::NSApplicationActivationOptions;

    let result = id_string.split('_').collect::<Vec<&str>>();

    if result.len() != 2 {
        return Err(anyhow!("invalid window id"));
    }
    let process_id = result[0].parse::<u32>()?;
    let _window_id = result[1].parse::<u64>()?;

    unsafe {
        let app_class = class!(NSRunningApplication);
        let app_with_process_id: *mut AnyObject = msg_send![
            app_class,
            runningApplicationWithProcessIdentifier: process_id as i64
        ];
        if !app_with_process_id.is_null() {
            #[allow(deprecated)]
            let options: u64 = NSApplicationActivationOptions::ActivateIgnoringOtherApps
                .bits()
                .try_into()
                .unwrap();
            let _: bool = msg_send![
                app_with_process_id,
                activateWithOptions: options
            ];
            return Ok(true);
        }
    }
    Ok(false)
}

#[cfg(target_os = "linux")]
#[pyframe_api]
fn get_active_window_id() -> Result<Option<String>> {
    // Unter Linux noch nicht implementiert
    Ok(None)
}

#[cfg(target_os = "linux")]
#[pyframe_api]
fn focus_by_window_id(_id_string: String) -> Result<bool> {
    // Unter Linux noch nicht implementiert
    Ok(false)
}

#[cfg(target_os = "windows")]
#[pyframe_api]
fn get_active_window_id() -> Result<String> {
    use winapi::um::winuser::GetForegroundWindow;

    let hwnd = unsafe { GetForegroundWindow() as usize };
    Ok(hwnd.to_string())
}

#[cfg(target_os = "windows")]
#[pyframe_api]
fn focus_by_window_id(hwnd_str: String) -> Result<()> {
    use winapi::shared::windef::HWND;
    use winapi::um::winuser::SetForegroundWindow;

    let hwnd = hwnd_str.parse::<usize>()? as HWND;
    unsafe {
        SetForegroundWindow(hwnd);
    }
    Ok(())
}
