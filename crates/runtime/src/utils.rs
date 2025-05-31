// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::{options::SocketSettings, CoreApplication};
use anyhow::{anyhow, Result};
use serde_json::{to_string_pretty, Value};
use std::{
    collections::HashMap,
    io::{BufRead as _, Write},
    sync::{Arc, Mutex},
};
use std::{
    fmt::{Debug, Formatter},
    ops::Deref,
    pin::Pin,
};
use tao::event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy, EventLoopWindowTarget};
pub type FrameEventLoop = EventLoop<UserEvent>;
pub type FrameEventLoopBuilder = EventLoopBuilder<UserEvent>;
pub type FrameEventLoopProxy = EventLoopProxy<UserEvent>;
pub type FrameWindowTarget = EventLoopWindowTarget<UserEvent>;

pub type FrameCallback = Pin<Box<dyn Fn(&FrameWindowTarget, &mut ControlFlow) -> Result<()> + Send>>;
pub struct FrameEvent(FrameCallback);

pub enum UserEvent {
    FrameEvent(FrameEvent),
    MenuEvent(muda::MenuEvent),
}

impl Debug for FrameEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FrameEvent").finish()
    }
}

impl FrameEvent {
    pub fn new<F: Fn(&FrameWindowTarget, &mut ControlFlow) -> Result<()> + Send + 'static>(f: F) -> Self {
        Self(Box::pin(f))
    }
}

impl Deref for FrameEvent {
    type Target = Pin<Box<dyn Fn(&FrameWindowTarget, &mut ControlFlow) -> Result<()> + Send>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type ArcMut<T> = Arc<Mutex<T>>;

pub fn arc<T>(t: T) -> Arc<T> {
    Arc::new(t)
}

pub fn arc_mut<T>(t: T) -> ArcMut<T> {
    Arc::new(Mutex::new(t))
}

pub struct IdCounter {
    next_id: u8,
}

impl IdCounter {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    pub fn next<T>(&mut self, excludes: &HashMap<u8, T>) -> Result<u8> {
        for _ in 0..u8::MAX {
            let id = self.next_id;
            if excludes.contains_key(&id) {
                self.next_id += 1;
                continue;
            }
            return Ok(id);
        }
        Err(anyhow!("Failed to find a valid id."))
    }
}
impl Default for IdCounter {
    fn default() -> Self {
        Self::new()
    }
}
#[macro_export]
macro_rules! unsafe_impl_sync_send {
    ($type:ty) => {
        unsafe impl Send for $type {}
        unsafe impl Sync for $type {}
    };
}

#[macro_export]
macro_rules! set_property_some {
    ($builder:ident, $property:ident, &$value:expr) => {
        if let Some(value) = &$value {
            $builder = $builder.$property(value);
        }
    };
    ($builder:ident, $property:ident, $value:expr) => {
        if let Some(value) = $value {
            $builder = $builder.$property(value.clone());
        }
    };
}

#[macro_export]
macro_rules! set_property {
    ($builder:ident, $property:ident, $value:expr) => {
        $builder = $builder.$property($value);
    };
}

#[macro_export]
macro_rules! lock {
    ($value:expr) => {
        $value
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to lock {}.", stringify!($value)))
    };
}

#[macro_export]
macro_rules! lock_force {
    ($value:expr) => {
        $value.lock().unwrap()
    };
}

#[macro_export]
macro_rules! logical {
    ($window:expr, $method:ident) => {
        $window.$method().to_logical::<f64>($window.scale_factor())
    };

    ($window:expr, $item:expr, $method:ident) => {
        $item.$method().to_logical::<f64>($window.scale_factor())
    };
}

#[macro_export]
macro_rules! logical_try {
    ($window:expr, $method:ident) => {
        $window.$method()?.to_logical::<f64>($window.scale_factor())
    };
}

#[macro_export]
macro_rules! log_if_err {
    ($result:expr) => {
        if let Err(e) = $result {
            println!("[Error]: {}", e);
        }
    };
}

#[macro_export]
macro_rules! log {
    ($result:expr) => {
        println!("[Info]: {}", $result);
    };
}

#[macro_export]
macro_rules! log_err {
    ($result:expr) => {
        println!("[Error]: {}", $result);
    };
}

pub fn merge_values(dest: Value, src: Value) -> Value {
    match (dest, src) {
        (Value::Null, src) => src,
        (dest, Value::Null) => dest,
        (Value::Object(mut dest_map), Value::Object(src_map)) => {
            for (key, src_val) in src_map {
                let dest_val = dest_map.entry(key).or_insert(Value::Null);
                *dest_val = merge_values(dest_val.take(), src_val);
            }
            Value::Object(dest_map)
        }
        (_, src) => src,
    }
}

// pub fn try_or_log_err<F, T>(mut func: F) where F: FnMut() -> Result<T> {
//     match func() {
//         Ok(_) => {}
//         Err(e) => {
//             log_err!(e);
//         }
//     }
// }

#[macro_export]
macro_rules! try_or_log_err {
    ($body:block ) => {
        match (move || -> anyhow::Result<()> { $body })() {
            Ok(_) => {}
            Err(e) => {
                $crate::log_err!(e);
            }
        }
    };
}
pub fn url_join(left: &str, right: &str) -> String {
    if right.is_empty() {
        left.to_string()
    } else if left.ends_with("/") {
        format!("{}{}", left, right)
    } else {
        format!("{}/{}", left, right)
    }
}

pub fn merge_id(window_id: u8, item_id: u8) -> u16 {
    ((window_id as u16) << 8) | (item_id as u16)
}

pub fn split_id(merged_id: u16) -> (u8, u8) {
    ((merged_id >> 8) as u8, merged_id as u8)
}

#[allow(dead_code)]
pub fn get_json_sync(url: &str) -> anyhow::Result<serde_json::Value> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .send()? // blockierender Aufruf
        .error_for_status()? // wirft bei HTTP-Fehlern
        .json()?; // wandelt in JSON (serde_json::Value) um

    Ok(response)
}

#[allow(dead_code)]
pub fn fetch_json_struct<T: serde::de::DeserializeOwned>(url: &str) -> anyhow::Result<T> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .send()?
        .error_for_status()? // HTTP 4xx / 5xx werfen Fehler
        .json::<T>()?; // Deserialisierung in das Struct

    Ok(response)
}

#[allow(dead_code)]
pub fn stop_pyruntime() {
    stdout_handler(&serde_json::json!({ "event": "stop"}));
}

#[allow(dead_code)]
pub fn stdout_handler<S: serde::Serialize>(data: &S) {
    let json = to_json(data);
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{}", json).unwrap();
    handle.flush().unwrap();
}

fn to_json<S: serde::Serialize>(data: &S) -> String {
    serde_json::to_string(data).unwrap_or_else(|_| "{}".to_string())
}

pub fn read_json_from_stdin() -> anyhow::Result<Value> {
    let stdin = std::io::stdin();
    let mut line = String::new();

    // Zeile einlesen oder sofort crashen
    stdin
        .lock()
        .read_line(&mut line)
        .expect("[RUST] Fehler beim Lesen von stdin");

    // JSON parsen oder crashen
    let data = serde_json::from_str(&line).expect("[RUST] Ungültiges JSON in stdin  Parsing fehlgeschlagen");
    Ok(data)
}

#[cfg(target_os = "windows")]
pub fn make_base_url(protocol: &str, host: &str) -> String {
    format!("http://{}.{}", protocol, host)
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
pub fn make_base_url(protocol: &str, host: &str) -> String {
    format!("{}://{}", protocol, host)
}

pub fn get_host_from_url(url: &str) -> Option<String> {
    let url = url::Url::parse(url).ok()?;
    let scheme = url.scheme();
    let host = url.host_str()?;
    Some(format!("{}://{}", scheme, host))
}

/*

/// Wandelt Option<T> in JSON um oder `"null"` wenn nicht gesetzt
fn opt_to_js<T: serde::Serialize>(opt: &Option<T>) -> String {
    match opt {
        Some(v) => to_string_pretty(v).unwrap_or_else(|_| "null".into()),
        None => "null".into(),
    }
}
 */
/// Wandelt Option<T> in JS-Objektfeld oder nichts, wenn None
fn opt_to_field<T: serde::Serialize>(key: &str, opt: &Option<T>) -> Option<String> {
    opt.as_ref()
        .map(|v| format!(r#"  {}: {}"#, key, to_string_pretty(v).unwrap_or("null".into())))
}

/// Generiert vollständigen Socket.IO-Client-JavaScript-Code
pub fn generate_socketio_js(config: Option<SocketSettings>, settings: crate::options::AppOptions) -> String {
    let mut options: Vec<String> = vec![];

    // Hilfs-Makro zum Hinzufügen von Optionen, falls vorhanden
    macro_rules! add {
        ($field:ident) => {
            if let Some(conf) = &config {
                if let Some(line) = opt_to_field(stringify!($field), &conf.$field) {
                    options.push(line);
                }
            }
        };
    }

    // IO Factory Optionen
    add!(force_new);
    add!(multiplex);

    // Engine.IO Optionen
    add!(add_trailing_slash);
    add!(auto_unref);
    add!(close_on_beforeunload);
    add!(extra_headers);
    add!(force_base64);
    add!(protocols);
    add!(query);
    add!(remember_upgrade);
    add!(timestamp_param);
    add!(timestamp_requests);
    add!(transport_options);
    add!(transports);
    add!(try_all_transports);
    add!(upgrade);
    add!(with_credentials);

    // Manager Optionen
    add!(auto_connect);
    add!(parser);
    add!(randomization_factor);
    add!(reconnection);
    add!(reconnection_attempts);
    add!(reconnection_delay);
    add!(reconnection_delay_max);
    add!(timeout);

    // Socket Optionen
    add!(ack_timeout);
    add!(auth);
    add!(retries);

    // Standardfelder setzen (Host, Port, Pfad)
    let base_url = format!("{}:{}", settings.host, settings.port);
    let path_str = config
        .as_ref()
        .and_then(|c| c.path.as_ref())
        .map(|p| p.trim().trim_start_matches('/')) // trim
        .filter(|p| !p.is_empty()) // leere Strings filtern
        .unwrap_or("socket.io");

    // Optionen-Block für JavaScript-Objekt generieren
    let options_block = if options.is_empty() {
        "  // keine zusätzlichen Optionen gesetzt".to_string()
    } else {
        options.join(",\n")
    };

    // JavaScript-Code als String formatieren
    format!(
        r#"
const url = (window.location.protocol === "https:") ? "wss://" : "ws://";
window.socket = io(url + "{base_url}", {{
  path: "/_pyframe_ws/{path_str}",
{options_block}
}});

window.socket.on("connect", () => {{
  console.log("[Socket.IO] Verbunden mit " + url + "{base_url}/{path_str}");
}});

window.socket.on("disconnect", () => {{
  console.log("[Socket.IO] Verbindung getrennt");
}});

window.socket.on("connect_error", (err) => {{
  console.error("[Socket.IO] Verbindungsfehler:", err);
}});
"#,
        base_url = base_url,
        path_str = path_str,
        options_block = options_block
    )
}

#[allow(dead_code)]
fn send_json_sync<S: serde::Serialize>(url: &str, path: &str, data: &S) -> anyhow::Result<serde_json::Value> {
    let server_url = format!("{}/{}", url, path);
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(server_url)
        .json(data) // <- Wichtig: Body setzen
        .send()? // blockierender Aufruf
        .error_for_status()? // wirft bei HTTP-Fehlern
        .json()?; // wandelt Antwort in JSON um

    Ok(response)
}

#[allow(dead_code)]
pub fn menu_provider(
    app: &Arc<CoreApplication>,
    window: tao::window::Window,
) -> anyhow::Result<tao::window::Window> {
    let app = app.clone();
    let menu_mode = app.launch_info.options.menu_mode.clone();
    let config = app.launch_info.options.window_menu.clone();
    match menu_mode {
        Some(crate::options::MenuMode::Menu) | Some(crate::options::MenuMode::MenuAndTray) => {
            if let Some(menu_frame) = &config {
                if menu_frame.has_menu_item() {
                    let mut menu_sys_guard = app.menu()?;
                    menu_sys_guard.register_menu_items(menu_frame.clone())?;
                    let menu_bar = menu_sys_guard.get_menu_manager()?;

                    #[cfg(target_os = "windows")]
                    unsafe {
                        use tao::platform::windows::WindowExtWindows;
                        let _ = menu_bar.init_for_hwnd(window.hwnd() as _);
                    }
                    #[cfg(target_os = "linux")]
                    {
                        use tao::platform::unix::WindowExtUnix;
                        let _ = menu_bar.init_for_gtk_window(window.gtk_window(), window.default_vbox());
                    }
                    #[cfg(target_os = "macos")]
                    {
                        menu_bar.init_for_nsapp();
                    }
                } else {
                    println!("window_menu ist leer, kein Menü wird angelegt.");
                }
            }
            let cloned_proxy = app.proxy.clone();
            muda::MenuEvent::set_event_handler(Some(move |event| {
                let _ = cloned_proxy.send_event(UserEvent::MenuEvent(event));
            }));
        }
        Some(crate::options::MenuMode::Tray) => {
            println!("Starte Fenster mit Tray-Icon (ohne Menü im Fenster).");
            // Hier keine Menü-Initialisierung nötig!
        }
        _ => {
            eprintln!("Unbekannter MenuMode – es wird kein Menü oder Tray-Icon erstellt!");
        }
    }

    Ok(window)
}
