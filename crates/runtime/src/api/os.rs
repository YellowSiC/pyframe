// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use anyhow::Result;
use directories::UserDirs;
use pyframe_macros::pyframe_api;
use serde_json::{json, Value};
use sys_locale::get_locale;

use crate::api_manager::ApiManager;

pub fn register_api_instances(_api_manager: &mut ApiManager) {
    _api_manager.register_api("os.info", info);
    _api_manager.register_api("os.dirs", dirs);
    _api_manager.register_api("os.sep", sep);
    _api_manager.register_api("os.eol", eol);
    _api_manager.register_api("os.locale", locale);
}

#[pyframe_api]
fn info() -> Result<Value> {
    let info = os_info::get();
    Ok(json!({
        "os": info.os_type().to_string(),
        "arch": std::env::consts::ARCH.to_string(),
        "version": info.version().to_string(),
    }))
}

#[pyframe_api]
fn dirs() -> Result<Value> {
    let user_dirs = UserDirs::new();

    match user_dirs {
        Some(user_dirs) => Ok(json!({
            "temp": app.launch_info.temp_dir,
            "data": app.launch_info.data_dir,

            "home": user_dirs.home_dir(),
            "audio": user_dirs.audio_dir(),
            "desktop": user_dirs.desktop_dir(),
            "document": user_dirs.document_dir(),
            "download": user_dirs.download_dir(),
            "font": user_dirs.font_dir(),
            "picture": user_dirs.picture_dir(),
            "public": user_dirs.public_dir(),
            "template": user_dirs.template_dir(),
            "video": user_dirs.video_dir(),
        })),
        None => Ok(json!({
            "temp": app.launch_info.temp_dir,
            "data": app.launch_info.data_dir,
        })),
    }
}

#[pyframe_api]
fn sep() -> Result<String> {
    Ok(std::path::MAIN_SEPARATOR.to_string())
}

#[pyframe_api]
fn eol() -> Result<String> {
    #[cfg(target_os = "windows")]
    let eol = "\r\n";

    #[cfg(not(target_os = "windows"))]
    let eol = "\n";

    Ok(eol.to_string())
}

#[pyframe_api]
fn locale() -> Result<String> {
    Ok(get_locale().unwrap_or("en-US".to_string()))
}
