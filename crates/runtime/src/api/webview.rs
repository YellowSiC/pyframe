// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::{api_manager::ApiManager, utils::make_base_url};
use anyhow::Result;
use pyframe_macros::{pyframe_api, pyframe_event_api};

pub fn register_api_instances(_api_manager: &mut ApiManager) {
    // Basis-APIs
    _api_manager.register_api("webview.baseUrl", base_url);
    _api_manager.register_api("webview.baseFileSystemUrl", base_filesystem_url);

    // Event-APIs
    _api_manager.register_event_api("webview.isDevtoolsOpen", is_devtools_open);
    _api_manager.register_event_api("webview.openDevtools", open_devtools);
    _api_manager.register_event_api("webview.closeDevtools", close_devtools);
    _api_manager.register_event_api("webview.bounds", bounds);
    _api_manager.register_event_api("webview.clearAllBrowsingData", clear_all_browsing_data);
    _api_manager.register_event_api("webview.cookies", cookies);
    _api_manager.register_event_api("webview.cookiesForUrl", cookies_for_url);
    _api_manager.register_event_api("webview.evaluateScript", evaluate_script);
    _api_manager.register_event_api("webview.focus", focus);
    _api_manager.register_event_api("webview.loadHtml", load_html);
    _api_manager.register_event_api("webview.loadUrl", load_url);
    _api_manager.register_event_api("webview.zoom", zoom);
    _api_manager.register_event_api("webview.print", print);
    _api_manager.register_event_api("webview.reload", reload);
    _api_manager.register_event_api("webview.url", url);
    _api_manager.register_event_api("webview.setBackgroundColor", set_background_color);
    _api_manager.register_event_api("webview.setBounds", set_bounds);
    _api_manager.register_event_api("webview.visible", visible);
    _api_manager.register_event_api("webview.loadUrlWithHeaders", load_url_with_headers);

    // Optional/Plattform-spezifische Event-APIs
    #[cfg(not(target_os = "android"))]
    _api_manager.register_event_api("webview.focusParent", focus_parent);

    #[cfg(target_os = "windows")]
    {
        _api_manager.register_event_api("webview.reparent", reparent);
        _api_manager.register_event_api("webview.setMemoryUsageLevel", set_memory_usage_level);
        _api_manager.register_event_api("webview.controller", cotroller);
        _api_manager.register_event_api("webview.setTheme", set_theme);
    }
}

#[pyframe_event_api]
fn is_devtools_open() -> Result<bool> {
    Ok(window.webview.is_devtools_open())
}

#[pyframe_event_api]
fn open_devtools() -> Result<()> {
    window.webview.open_devtools();
    Ok(())
}
#[pyframe_event_api]
fn close_devtools() -> Result<()> {
    window.webview.close_devtools();
    Ok(())
}

#[pyframe_event_api]
fn bounds() -> Result<()> {
    window.webview.bounds()?;
    Ok(())
}

#[pyframe_event_api]
fn clear_all_browsing_data() -> Result<()> {
    window.webview.clear_all_browsing_data()?;
    Ok(())
}

#[pyframe_event_api]
fn cookies() -> Result<()> {
    window.webview.cookies()?;
    Ok(())
}

#[pyframe_event_api]
fn cookies_for_url(url: String) -> Result<()> {
    window.webview.cookies_for_url(&url)?;
    Ok(())
}

#[pyframe_event_api]
fn evaluate_script(code: String) -> Result<()> {
    window.webview.evaluate_script(&code)?;
    Ok(())
}

#[pyframe_event_api]
fn focus() -> Result<()> {
    window.webview.focus()?;
    Ok(())
}

#[cfg(not(target_os = "android"))]
#[pyframe_event_api]
fn focus_parent() -> Result<()> {
    window.webview.focus_parent()?;
    Ok(())
}

#[pyframe_event_api]
fn webview_id() -> Result<()> {
    window.webview.id();
    Ok(())
}

#[pyframe_event_api]
fn load_html(code: String) -> Result<()> {
    window.webview.load_html(&code)?;
    Ok(())
}

#[pyframe_event_api]
fn load_url(url: String) -> Result<()> {
    window.webview.load_url(&url)?;
    Ok(())
}

#[pyframe_event_api]
fn zoom(scale: f64) -> Result<()> {
    // Plattformabhängige Behandlung
    #[cfg(target_os = "android")]
    {
        Err(anyhow::anyhow!("Zoom wird auf Android nicht unterstützt."))
    }
    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "windows", target_os = "linux"))]
    {
        window.webview.zoom(scale)?;
        Ok(())
    }
}

#[pyframe_event_api]
fn print() -> Result<()> {
    window.webview.print()?;
    Ok(())
}

#[pyframe_event_api]
fn reload() -> Result<()> {
    window.webview.reload()?;
    Ok(())
}

#[pyframe_event_api]
fn url() -> Result<()> {
    window.webview.url()?;
    Ok(())
}

#[pyframe_event_api]
fn set_background_color(r: u8, g: u8, b: u8, a: Option<u8>) -> Result<()> {
    if cfg!(any(target_os = "macos", target_os = "ios")) {
        return Err(anyhow::anyhow!(
            "set_background_color wird auf macOS/iOS nicht unterstützt."
        ));
    }

    let rgba = (r, g, b, a.unwrap_or(255)); // als Tupel, nicht als Struct

    window.webview.set_background_color(rgba)?;
    Ok(())
}

#[pyframe_event_api]
fn set_bounds(x: i32, y: i32, width: i32, height: i32) -> Result<()> {
    if cfg!(target_os = "linux") {
        let rect = wry::Rect {
            position: tao::dpi::Position::Logical((x, y).into()),
            size: tao::dpi::Size::Logical((width, height).into()),
        };

        window.webview.set_bounds(rect)?;
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "set_bounds wird auf dieser Plattform nicht unterstützt."
        ))
    }
}

#[cfg(target_os = "windows")]
#[pyframe_event_api]
fn reparent(hwnd: isize) -> Result<()> {
    use wry::WebViewExtWindows;
    window.webview.reparent(hwnd)?;
    Ok(())
}

#[pyframe_event_api]
fn visible(visible: bool) -> Result<()> {
    window.webview.set_visible(visible)?;
    Ok(())
}

#[cfg(target_os = "windows")]
#[pyframe_event_api]
fn set_memory_usage_level(level: String) -> Result<()> {
    use wry::{MemoryUsageLevel, WebViewExtWindows};

    // String -> MemoryUsageLevel
    let level_enum = match level.to_lowercase().as_str() {
        "low" => MemoryUsageLevel::Low,
        "normal" => MemoryUsageLevel::Normal,
        _ => {
            return Err(anyhow::anyhow!(
                "Ungültiger Speicherlevel: {}. Erlaubt: low, normal",
                level
            ));
        }
    };

    window.webview.set_memory_usage_level(level_enum)?;
    Ok(())
}
#[cfg(target_os = "windows")]
#[pyframe_event_api]
fn cotroller() -> Result<()> {
    use wry::WebViewExtWindows;
    window.webview.controller();
    Ok(())
}

#[cfg(target_os = "windows")]
#[pyframe_event_api]
fn set_theme(theme: bool) -> Result<()> {
    use wry::{Theme, WebViewExtWindows};
    // bool -> Theme
    let theme = if theme { Theme::Dark } else { Theme::Light };

    window.webview.set_theme(theme)?;
    Ok(())
}

#[pyframe_event_api]
fn load_url_with_headers(url: String, headers_json: Option<serde_json::Value>) -> Result<()> {
    let mut headers = wry::http::HeaderMap::new();

    if let Some(serde_json::Value::Object(map)) = headers_json {
        for (key, value) in map {
            if let (Ok(header_name), Some(header_value)) =
                (wry::http::HeaderName::try_from(key.as_str()), value.as_str())
            {
                if let Ok(hv) = wry::http::HeaderValue::from_str(header_value) {
                    headers.insert(header_name, hv);
                } else {
                    println!("Ungültiger Headerwert für '{}'", key);
                }
            } else {
                println!("Ungültiger Headername oder -wert für '{}'", key);
            }
        }
    }

    window.webview.load_url_with_headers(&url, headers)?;
    Ok(())
}

#[pyframe_api]
fn base_url() -> Result<String> {
    Ok(make_base_url("pyframe", &app.launch_info.id_name))
}

#[pyframe_api]
fn base_filesystem_url() -> Result<String> {
    Ok(make_base_url("pyframe", "filesystem"))
}
