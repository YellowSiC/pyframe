// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

mod thread_pool;
use self::thread_pool::ThreadPool;
use crate::{
    lock_force,
    options::AppOptions,
    unsafe_impl_sync_send,
    utils::{arc_mut, ArcMut, FrameWindowTarget},
    window_manager::window::FrameWindow,
    CoreApplication,
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashMap, pin::Pin, sync::Arc};
use tao::{event_loop::ControlFlow, window::Window};

type EventApiFunc<T> =
    fn(Arc<CoreApplication>, Arc<FrameWindow>, ApiRequest, &FrameWindowTarget, &mut ControlFlow) -> Result<T>;

#[derive(Debug, Deserialize, Clone)]
pub struct ApiArguments(Value);

impl ApiArguments {
    pub fn single<T: serde::de::DeserializeOwned>(&self) -> Result<T> {
        Ok(serde_json::from_value::<(T,)>(self.0.clone())?.0)
    }

    pub fn get<T: serde::de::DeserializeOwned>(&self) -> Result<T> {
        Ok(serde_json::from_value(self.0.clone())?)
    }

    pub fn optional<T: serde::de::DeserializeOwned>(&self, args_size: usize) -> Result<T> {
        let mut args = serde_json::from_value::<Vec<serde_json::Value>>(self.0.clone())?;
        args.resize(args_size, json!(null));
        let args = json!(args);
        Ok(serde_json::from_value(args)?)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApiRequest(pub u8, pub String, pub ApiArguments);

impl ApiRequest {
    pub fn err<C: Into<i32>, S: Into<String>>(&self, code: C, msg: S) -> ApiResponse {
        ApiResponse(self.0, code.into(), msg.into(), json!(null))
    }

    pub fn ok<D: Serialize>(&self, data: D) -> ApiResponse {
        ApiResponse(self.0, 0, "ok".to_string(), json!(data))
    }

    pub fn args(&self) -> &ApiArguments {
        &self.2
    }
}

pub type Code = i32;

#[derive(Debug, Serialize, Clone)]
pub struct ApiResponse(u8, Code, String, Value);

pub type ApiInstance = Pin<Box<dyn Fn(Arc<CoreApplication>, Arc<FrameWindow>, ApiRequest) -> Result<()>>>;

unsafe_impl_sync_send!(ApiManager);
pub struct ApiManager {
    app: Option<Arc<CoreApplication>>,
    thread_pool: ArcMut<ThreadPool>,
    api_instance: HashMap<String, ApiInstance>,
}

impl ApiManager {
    pub fn new(options: &AppOptions) -> ArcMut<ApiManager> {
        let workers = options.workers.unwrap_or(4);
        let thread_pool = ThreadPool::new(workers);
        arc_mut(ApiManager {
            app: None,
            thread_pool,
            api_instance: HashMap::new(),
        })
    }

    pub fn bind_app(&mut self, app: Arc<CoreApplication>) {
        self.app = Some(app);
    }

    pub fn register_async_api<S: Into<String>, T: Serialize + 'static>(
        &mut self,
        name: S,
        api_func: fn(Arc<CoreApplication>, Arc<FrameWindow>, ApiRequest) -> Result<T>,
    ) {
        let thread_pool = self.thread_pool.clone();
        let api_instance: ApiInstance = Box::pin(move |app, window, request| {
            lock_force!(thread_pool).run(move || {
                let result = api_func(app.clone(), window.clone(), request.clone());
                let response = match result {
                    Ok(data) => request.ok(data),
                    Err(err) => request.err(-1, err.to_string()),
                };
                window.send_ipc_callback(response)
            })
        });
        self.api_instance.insert(name.into(), api_instance);
    }

    pub fn register_api<S: Into<String>, T: Serialize + 'static>(
        &mut self,
        name: S,
        api_func: fn(Arc<CoreApplication>, Arc<FrameWindow>, ApiRequest) -> Result<T>,
    ) {
        let api_instance: ApiInstance = Box::pin(move |app, window, request| {
            let result = api_func(app, window.clone(), request.clone());
            let response = match result {
                Ok(data) => request.ok(data),
                Err(err) => request.err(-1, err.to_string()),
            };
            window.send_ipc_callback(response)
        });
        self.api_instance.insert(name.into(), api_instance);
    }

    pub fn register_event_api<S: Into<String>, T: Serialize + 'static>(&mut self, name: S, api_func: EventApiFunc<T>) {
        let api_instance: ApiInstance = Box::pin(move |app, window, request| {
            window.clone().send_event(move |target, control_flow| {
                let result = api_func(app.clone(), window.clone(), request.clone(), target, control_flow);
                let response = match result {
                    Ok(data) => request.ok(data),
                    Err(err) => request.err(-1, err.to_string()),
                };
                window.clone().send_ipc_callback(response)?;
                Ok(())
            })
        });
        self.api_instance.insert(name.into(), api_instance);
    }

    pub fn call(&self, _window: &Window, request_str: String) -> Result<()> {
        let app = self.app.clone().ok_or(anyhow!("app not set"))?;
        let window = app.window()?.get_window_inner(_window.id())?;

        let request = serde_json::from_str::<ApiRequest>(&request_str)?;

        let api = self.api_instance.get(&request.1);

        if let Some(api_func) = api {
            let result = api_func(app, window.clone(), request.clone());

            if let Err(err) = result {
                window.send_ipc_callback(request.err(-1, err.to_string()))?;
                return Err(err);
            }

            Ok(())
        } else {
            window.send_ipc_callback(request.err(-1, "api not found".to_string()))?;
            Err(anyhow!("api not found"))
        }
    }
}
