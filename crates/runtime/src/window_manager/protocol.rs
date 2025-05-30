// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use serde_json::Value;
use wry::http::Request;

use crate::utils::{get_host_from_url, make_base_url, url_join};

pub fn ipc_listener(app: &std::sync::Arc<crate::CoreApplication>) -> impl Fn(Request<String>) + 'static {
    let _cloned_app = app.clone();

    move |req: Request<String>| {
        // JSON-Body parsen
        let body: Value = match serde_json::from_slice(req.body().as_bytes()) {
            Ok(val) => val,
            Err(err) => {
                log::info!("[IPC] Invalid JSON body: {:?}", err);
                return;
            }
        };

        // Header extrahieren
        let headers = req
            .headers()
            .iter()
            .map(|(k, v)| crate::options::HeaderData {
                key: k.as_str().to_string(),
                value: v.to_str().ok().map(str::to_string),
            })
            .collect::<Vec<_>>();

        // Nachricht zusammenbauen
        let _ipc_message = crate::options::IpcMessage {
            body,
            headers,
            method: req.method().to_string(),
            uri: req.uri().to_string(),
        };
    }
}

#[allow(dead_code)]
fn parse_py_response(data: &str) -> anyhow::Result<Value> {
    let parsed: Value = serde_json::from_str(data)?;

    if parsed.get("event").and_then(Value::as_str) == Some("empty") {
        Ok(Value::Null)
    } else {
        Ok(parsed)
    }
}

#[allow(clippy::op_ref)]
pub fn render_web_protocol(
    app: std::sync::Arc<crate::CoreApplication>,
    builder: wry::WebViewBuilder,
) -> anyhow::Result<wry::WebViewBuilder> {
    let id_name = app.launch_info.id_name.clone();
    let entry = app.launch_info.options.window.entry.clone();
    let protocol = "pyframe";
    let debug_entry = app.launch_info.options.debug_entry.clone();
    let base_url = debug_entry.unwrap_or(make_base_url(protocol, &id_name));
    let entry_url = url_join(&base_url, &entry.clone().unwrap_or_default());

    let prefix = get_host_from_url(&entry_url).unwrap_or(base_url.to_owned());
    let custom_protocol_app = app.clone();
    let builder = builder
        .with_navigation_handler(move |url| url.starts_with(&prefix))
        .with_custom_protocol(protocol.to_string(), move |_, request| {
            let hostname = request.uri().host().unwrap_or(&id_name);

            let mut path = request.uri().path().to_string();

            if path.ends_with('/') {
                path += "index.html";
            }

            let result = (|| -> anyhow::Result<Vec<u8>> {
                if hostname == &id_name {
                    let path = path.strip_prefix('/').unwrap_or("index.html");
                    custom_protocol_app.resource().load(path)
                } else if hostname == "filesystem" {
                    let path = path.strip_prefix('/').unwrap_or("index.html");
                    let path = std::path::Path::new(path); // für alle Plattformen!
                    Ok(std::fs::read(path)?)
                } else {
                    Err(anyhow::anyhow!("Invalid hostname: {}", hostname))
                }
            })();

            let origin = get_host_from_url(&request.uri().to_string()).unwrap_or("*".to_string());

            match result {
                Err(err) => wry::http::Response::builder()
                    .status(404)
                    .header(wry::http::header::CONTENT_TYPE, "text/plain; charset=utf-8")
                    .body(std::borrow::Cow::Owned(err.to_string().into_bytes()))
                    .unwrap(),

                Ok(content) => {
                    let mime_type = mime_guess::from_path(path)
                        .first()
                        .unwrap_or(mime_guess::mime::TEXT_PLAIN)
                        .to_string();

                    wry::http::Response::builder()
                        .status(200)
                        .header(wry::http::header::CONTENT_TYPE, mime_type)
                        .header("Access-Control-Allow-Origin", origin)
                        .body(std::borrow::Cow::Owned(content))
                        .unwrap()
                }
            }
        })
        .with_url(&entry_url);
    Ok(builder)
}

#[allow(clippy::op_ref)]
/// Baut die finale URL (aus Basis-URL und optionalem Pfad) und konfiguriert den `WebViewBuilder`.
/// Alle Navigationen innerhalb dieser URL werden erlaubt.
///
/// # Argumente
/// - `builder`: Ein `WebViewBuilder`, der weiterkonfiguriert wird.
/// - `app`: Geteilte Instanz der CoreApplication, um auf `launch_info` zuzugreifen.
///
/// # Rückgabe
/// - `anyhow::Result<wry::WebViewBuilder<'a>>` mit dem konfigurierten Builder.
pub fn build_full_url(
    builder: wry::WebViewBuilder,
    app: std::sync::Arc<crate::CoreApplication>,
) -> anyhow::Result<wry::WebViewBuilder> {
    // Hole Basis-URL (debug_entry) oder verwende den Standard
    let debug_entry = app.launch_info.options.debug_entry.clone();
    let default_url = "http://localhost:8080";
    let mut url = url::Url::parse(&debug_entry.unwrap_or_else(|| default_url.to_owned()))?;

    // Hole optionalen Pfad (entry) und setze ihn, falls vorhanden
    if let Some(p) = app.launch_info.options.window.entry.clone() {
        let clean_path = p.trim_start_matches('/'); // Entfernt führende Slashes
        url.set_path(clean_path);
    }

    // Finale URL als String
    let server_url = url.to_string();

    // Die Navigation-Handler-Logik: Nur eigene URL erlauben
    let allow_url = url.clone(); // für den Vergleich in der Closure

    let builder = builder.with_url(server_url).with_navigation_handler(move |uri| {
        // Parse die URI der Navigationsanfrage
        if let Ok(nav_url) = url::Url::parse(&uri) {
            // Erlaube nur Navigieren zu derselben Host/Port-Kombination
            nav_url.origin() == allow_url.origin()
        } else {
            false // Ungültige URL -> blocken
        }
    });

    Ok(builder)
}
