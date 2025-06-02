// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::api_manager::ApiManager;

use anyhow::Result;

use notify_rust::Notification;

use pyframe_macros::pyframe_event_api;


macro_rules! set_property {
    ($builder:expr, $method:ident, $value:expr) => {
        $builder.$method($value);
    };
}




pub fn register_api_instances(_api_manager: &mut ApiManager) {
    
    {
        _api_manager.register_event_api("controlcenter.notification", notification);
    }
}


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

    let mut notify = Notification::new();
    

    // set_property (immer)
    set_property!(notify, summary, &summary);

    if let Some(body) = body {
        notify.body(&body);
    }
    if let Some(app_id) = app_id {
        notify.app_id(&app_id);
    }

    if let Some(appname) = appname {
        notify.appname(&appname);
    }

    if let Some(path) = image_path {
        notify.image_path(&path);
    }
        // auto_icon als bool prüfen
    if let Some(sound) = sound_name {
        notify.sound_name(&sound);
    }
    // auto_icon als bool prüfen
    if let Some(subtitle) = subtitle {
        notify.subtitle(&subtitle);
    }


    if let Some(id) = &id {
        notify.id(*id); // Hier das Dereferenzieren!
    }
    // auto_icon als bool prüfen
    if let Some(true) = auto_icon {
        notify.auto_icon();
    }

    if let Some(icon) = icon {
        notify.icon(&icon);
    }

        // auto_icon als bool prüfen
    if let Some(timeout) = timeout {
        notify.timeout(timeout);
    }
    // Action, falls vorhanden
    if let Some((identifier, label)) = &action {
        notify.action(identifier, label);
    }
    

    // Zeige die notifyication
    notify.show()?;

    Ok(())
}
