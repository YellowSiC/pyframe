// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::api_manager::ApiManager;
use anyhow::Result;
use pyframe_macros::pyframe_api;
use serde_json::Value;

// Nutze dein http-Modul für IP-Location
pub fn register_api_instances(_api_manager: &mut ApiManager) {
    _api_manager.register_async_api("location.get_current_position", get_current_position);
}

#[pyframe_api]
fn get_current_position() -> Result<Value> {
    // IP-Location-API (kostenloser Dienst, limitiert)
    let resp: Value = reqwest::blocking::get("https://ipapi.co/json/")?.json()?;

    let latitude = resp["latitude"].as_f64().unwrap_or(0.0);
    let longitude = resp["longitude"].as_f64().unwrap_or(0.0);
    let city = resp["city"].as_str().unwrap_or("").to_string();

    Ok(serde_json::json!({
        "latitude": latitude,
        "longitude": longitude,
        "city": city
    }))
}
