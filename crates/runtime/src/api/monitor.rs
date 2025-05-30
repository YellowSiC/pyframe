// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::{api_manager::ApiManager, logical};
use anyhow::{Ok, Result};
use pyframe_macros::pyframe_api;
use serde_json::{json, Value};
use tao::monitor::MonitorHandle;

pub fn register_api_instances(_api_manager: &mut ApiManager) {
    _api_manager.register_api("monitor.list", list);
    _api_manager.register_api("monitor.current", current);
    _api_manager.register_api("monitor.primary", primary);
    _api_manager.register_api("monitor.fromPoint", from_point);
}

fn monitor_to_value(monitor: MonitorHandle) -> Value {
    json!({
        "name": monitor.name(),
        "size": logical!(monitor, size),
        "position": logical!(monitor, position),
        "physicalSize": monitor.size(),
        "physicalPosition": monitor.position(),
        "scaleFactor": monitor.scale_factor(),
    })
}

#[pyframe_api]
fn list() -> Result<Vec<Value>> {
    Ok(window.available_monitors().map(monitor_to_value).collect())
}

#[pyframe_api]
fn current() -> Result<Value> {
    match window.current_monitor() {
        Some(monitor) => Ok(monitor_to_value(monitor)),
        None => Ok(json!(null)),
    }
}

#[pyframe_api]
fn primary() -> Result<Value> {
    match window.primary_monitor() {
        Some(monitor) => Ok(monitor_to_value(monitor)),
        None => Ok(json!(null)),
    }
}

#[pyframe_api]
fn from_point(x: f64, y: f64) -> Result<Value> {
    match window.monitor_from_point(x, y) {
        Some(monitor) => Ok(monitor_to_value(monitor)),
        None => Ok(json!(null)),
    }
}
