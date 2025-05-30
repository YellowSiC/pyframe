// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
#[cfg(target_os = "windows")]
use crate::api_manager::ApiManager;
#[cfg(target_os = "windows")]
use anyhow::Result;
#[cfg(target_os = "windows")]
use notify_rust::Notification;
#[cfg(target_os = "windows")]
use pyframe_macros::pyframe_event_api;

#[cfg(target_os = "windows")]
macro_rules! set_property {
    ($builder:expr, $method:ident, $value:expr) => {
        $builder.$method($value);
    };
}

#[cfg(target_os = "windows")]
macro_rules! set_property_some {
    ($builder:expr, $method:ident, $value:expr) => {
        if let Some(v) = $value {
            $builder.$method(v);
        }
    };
}
#[cfg(target_os = "windows")]
pub fn register_api_instances(_api_manager: &mut ApiManager) {
    #[cfg(target_os = "windows")]
    {
        _api_manager.register_event_api("controlcenter.notification", notification);
    }
}

#[cfg(target_os = "windows")]
#[pyframe_event_api]
fn notification(
    summary: String,
    body: Option<String>,
    app_id: Option<String>,
    appname: Option<String>,
    icon: Option<String>,
    auto_icon: Option<bool>,
    image_path: Option<String>,
    sound_name: Option<String>,
    subtitle: Option<String>,
    timeout: Option<i32>,
    id: Option<u32>,
    action: Option<(String, String)>,
) -> Result<()> {
    // JSON → Rust-Struct

    let mut notif = Notification::new();

    // set_property (immer)
    set_property!(notif, summary, &summary);

    // set_property_some (nur wenn Option != None)
    set_property_some!(notif, body, &body);
    set_property_some!(notif, app_id, &app_id);
    set_property_some!(notif, appname, &appname);
    set_property_some!(notif, icon, &icon);
    set_property_some!(notif, image_path, &image_path);
    set_property_some!(notif, sound_name, &sound_name);
    set_property_some!(notif, subtitle, &subtitle);
    set_property_some!(notif, timeout, timeout);

    if let Some(id) = &id {
        notif.id(*id); // Hier das Dereferenzieren!
    }
    // auto_icon als bool prüfen
    if let Some(true) = auto_icon {
        notif.auto_icon();
    }

    // Action, falls vorhanden
    if let Some((identifier, label)) = &action {
        notif.action(identifier, label);
    }

    // Zeige die Notification
    notif.show()?;

    Ok(())
}
