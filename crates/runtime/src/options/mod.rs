// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

pub(crate) mod menu;
pub(crate) mod window;

use anyhow::Result;
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

// Gemeinsame Basiskonfiguration mit serde CamelCase-Transformation
#[derive(Debug, Clone, Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameShortcutOption {
    pub modifier: Option<crate::hylper::AcceleratorModifier>,
    pub key: crate::hylper::AcceleratorCode,
    pub accelerator_str: String,
    pub id: u8,
}

#[derive(Debug, Clone, Deserialize, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameTrayOptions {}

#[derive(Deserialize, Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ActivationPolicy {
    #[default]
    #[serde(rename = "regular")]
    Regular,

    #[serde(rename = "accessory")]
    Accessory,

    #[serde(rename = "prohibited")]
    Prohibited,
}

/// Background throttling policy
#[derive(Deserialize, Default, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FrameBackgroundThrottlingPolicy {
    /// A policy where background throttling is disabled
    #[default]
    #[serde(rename = "disabled")]
    Disabled,
    /// A policy where a web view that's not in a window fully suspends tasks.
    #[serde(rename = "suspend")]
    Suspend,
    /// A policy where a web view that's not in a window limits processing, but does not fully suspend tasks.
    #[serde(rename = "throttle")]
    Throttle,
}

impl From<FrameBackgroundThrottlingPolicy> for wry::BackgroundThrottlingPolicy {
    fn from(policy: FrameBackgroundThrottlingPolicy) -> Self {
        match policy {
            FrameBackgroundThrottlingPolicy::Disabled => wry::BackgroundThrottlingPolicy::Disabled,
            FrameBackgroundThrottlingPolicy::Suspend => wry::BackgroundThrottlingPolicy::Suspend,
            FrameBackgroundThrottlingPolicy::Throttle => wry::BackgroundThrottlingPolicy::Throttle,
        }
    }
}

#[derive(Debug, serde::Serialize, Deserialize, Clone)]
pub struct HeaderData {
    /// The key of the header.
    pub key: String,
    /// The value of the header.
    pub value: Option<String>,
}

#[derive(Debug, serde::Serialize, Deserialize, Clone)]
pub struct IpcMessage {
    /// The body of the message.
    pub body: serde_json::Value,
    /// The HTTP method of the message.
    pub method: String,
    /// The http headers of the message.
    pub headers: Vec<HeaderData>,
    /// The URI of the message.
    pub uri: String,
}

#[derive(Deserialize, Default, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppOptions {
    pub name: String,
    pub uuid: String,
    pub host: String,
    pub port: u16,
    pub icon: Option<String>,
    pub web_proto: Option<String>,
    pub internal_api: Option<bool>,
    pub debug_devtools: Option<bool>,
    pub debug_resource: Option<PathBuf>,
    pub debug_entry: Option<String>,
    // window options
    #[serde(default)]
    pub window: window::WindowConfig,
    pub workers: Option<u32>,
    #[cfg(target_os = "windows")]
    #[serde(flatten)]
    pub windows_extra: Option<crate::options::window::WindowsWindowConfig>,
    #[cfg(target_os = "linux")]
    #[serde(flatten)]
    pub linux_extra: Option<crate::options::window::LinuxWindowConfig>,
    #[cfg(target_os = "macos")]
    #[serde(flatten)]
    pub macos_extra: Option<crate::options::window::MacOSWindowConfig>,
    pub tray: Option<FrameTrayOptions>,
    pub shortcuts: Option<FrameShortcutOption>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocketSettings {
    pub force_new: Option<bool>,
    pub multiplex: Option<bool>,

    // Engine.IO Low-Level Options
    pub add_trailing_slash: Option<bool>,
    pub auto_unref: Option<bool>,
    pub close_on_beforeunload: Option<bool>,
    pub extra_headers: Option<HashMap<String, String>>,
    pub force_base64: Option<bool>,
    pub path: Option<String>,
    pub protocols: Option<Vec<String>>,
    pub query: Option<serde_json::Value>,
    pub remember_upgrade: Option<bool>,
    pub timestamp_param: Option<String>,
    pub timestamp_requests: Option<bool>,
    pub transport_options: Option<serde_json::Value>,
    pub transports: Option<Vec<String>>,
    pub try_all_transports: Option<bool>,
    pub upgrade: Option<bool>,
    pub with_credentials: Option<bool>,

    // Node.js-specific / Manager Options
    pub auto_connect: Option<bool>,
    pub parser: Option<String>,
    pub randomization_factor: Option<f64>,
    pub reconnection: Option<bool>,
    pub reconnection_attempts: Option<u32>,
    pub reconnection_delay: Option<u64>,
    pub reconnection_delay_max: Option<u64>,
    pub timeout: Option<u64>,

    // Socket Options
    pub ack_timeout: Option<u64>,
    pub auth: Option<serde_json::Value>,
    pub retries: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct LaunchInfo {
    pub id_name: String,
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub temp_dir: PathBuf,
    pub options: AppOptions,
    pub socket_settings: Option<SocketSettings>,
}
impl LaunchInfo {
    pub fn new(raw_data: String) -> Result<Self> {
        let raw: serde_json::Value = serde_json::from_str(&raw_data)?;
        let mut value = raw.clone();

        // Plattform-spezifische Konfiguration mergen
        let platform = std::env::consts::OS;
        if let Some(platform_data) = value.get(platform).cloned() {
            value = crate::utils::merge_values(value, platform_data);
        }

        // Erst AppOptions aus dem gemergten JSON lesen
        let mut options: AppOptions = serde_json::from_value(value.clone())?;

        // Wenn Devtools per CLI aktiviert sind
        if raw.get("debugDevtools").and_then(|v| v.as_bool()).unwrap_or(false) {
            options.window.webview_devtools = Some(true);
        }

        // Identifikation und Verzeichnisse vorbereiten
        let name = options.name.clone();
        let uuid = options.uuid.clone();
        let id_name = format!("{}_{}", name.to_lowercase(), &uuid[..8]);

        let base_dirs = BaseDirs::new().ok_or_else(|| anyhow::anyhow!("Could not determine user directories"))?;
        let temp_dir = std::env::temp_dir().join(&id_name);
        let data_dir = base_dirs.data_dir().join(&id_name);
        let cache_dir = base_dirs.cache_dir().join(&id_name);

        let socket_settings = if options.internal_api.unwrap_or(true) {
            let socket_host = options.host.clone();
            let socket_port = options.port;
            let server_url = format!("http://{}:{}/pyframe_socket_info", socket_host, socket_port);
            Some(crate::utils::fetch_json_struct::<SocketSettings>(&server_url)?)
        } else {
            None
        };

        Ok(Self {
            id_name,
            data_dir,
            cache_dir,
            temp_dir,
            options,
            socket_settings,
        })
    }
}
