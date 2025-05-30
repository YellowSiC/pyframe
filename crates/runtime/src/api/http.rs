// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use anyhow::Result;
use pyframe_macros::pyframe_api;
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::api_manager::ApiManager;

pub fn register_api_instances(_api_manager: &mut ApiManager) {
    _api_manager.register_async_api("http.request", request);
    _api_manager.register_async_api("http.get", get);
    _api_manager.register_async_api("http.post", post);
}

type Headers = HashMap<String, String>;

#[derive(Deserialize)]
struct RequestOptions {
    pub method: String,
    pub url: String,
    pub headers: Option<Headers>,
    pub body: Option<String>,
    pub proxy: Option<String>,
}

fn _request(options: RequestOptions) -> Result<Value> {
    let mut client_builder = Client::builder();

    // Proxy einstellen (wenn vorhanden)
    if let Some(proxy_url) = &options.proxy {
        let proxy = reqwest::Proxy::all(proxy_url)?;
        client_builder = client_builder.proxy(proxy);
    }

    let client = client_builder.build()?;

    // Header umwandeln
    let mut header_map = HeaderMap::new();
    if let Some(headers) = options.headers {
        for (key, value) in headers {
            let header_name = HeaderName::from_bytes(key.as_bytes())?;
            let header_value = HeaderValue::from_str(&value)?;
            header_map.insert(header_name, header_value);
        }
    }

    // Request vorbereiten
    let request_builder = client
        .request(options.method.as_str().parse()?, &options.url)
        .headers(header_map);

    // Body anhängen (optional)
    let response: Response = if let Some(body) = options.body {
        request_builder.body(body).send()?
    } else {
        request_builder.send()?
    };

    // Status & Header
    let status = response.status().as_u16();
    let headers = response
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or_default().to_string()))
        .collect::<HashMap<_, _>>();

    // Body
    let body = response.text()?;

    Ok(json!({
        "status": status,
        "headers": headers,
        "body": body,
    }))
}

#[pyframe_api]
fn request(options: RequestOptions) -> Result<Value> {
    _request(options)
}

#[pyframe_api]
fn get(url: String, headers: Option<Headers>) -> Result<Value> {
    _request(RequestOptions {
        method: "GET".to_string(),
        url,
        headers,
        body: None,
        proxy: None,
    })
}

#[pyframe_api]
fn post(url: String, body: String, headers: Option<Headers>) -> Result<Value> {
    _request(RequestOptions {
        method: "POST".to_string(),
        url,
        headers,
        body: Some(body),
        proxy: None,
    })
}
