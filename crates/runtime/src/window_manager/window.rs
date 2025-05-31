// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use super::{builder::FrameBuilder, WindowManager};
use crate::{
    options::window::WindowConfig,
    unsafe_impl_sync_send,
    utils::{self, FrameEvent, FrameEventLoopProxy, FrameWindowTarget},
    CoreApplication,
};
use anyhow::Result;
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};
use tao::window::{Window, WindowId};
use wry::WebView;

#[allow(dead_code)]
pub struct FrameWindowState {
    pub is_block_closed_requested: bool,
}

unsafe_impl_sync_send!(FrameWindow);
impl Deref for FrameWindow {
    type Target = Window;
    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

#[allow(dead_code)]
pub struct FrameWindow {
    pub id: u8,
    pub window: Window,
    pub window_id: WindowId,
    pub webview: WebView,
    app: Arc<CoreApplication>,
    event_loop_proxy: FrameEventLoopProxy,
    pub state: Mutex<FrameWindowState>,
}

impl FrameWindow {
    pub fn new(
        app: Arc<CoreApplication>,
        id: u8,
        target: &FrameWindowTarget,
        options: WindowConfig,
        manager: &mut WindowManager,
    ) -> Result<Arc<FrameWindow>> {
        let window = FrameBuilder::build_window(&app, manager, id, &options, target)?;
        let (window, webview) = FrameBuilder::build_webview(&app, &options, window, &mut manager.web_context)?;

        let window = utils::menu_provider(&app, window)?;
        let window_id = window.id();

        Ok(crate::utils::arc(Self {
            app: app.clone(),
            id,
            window,
            window_id,
            webview,
            //menu: init_menu_bar,
            event_loop_proxy: app.proxy.clone(),
            state: Mutex::new(FrameWindowState {
                is_block_closed_requested: false,
            }),
        }))
    }
    pub fn post_message<P: serde::Serialize>(self: &Arc<Self>, payload: P) -> anyhow::Result<()> {
        // JSON-String des Payloads
        let payload = serde_json::to_string(&payload)?;
        let _self = self.clone();

        self.send_event(move |_, _| {
            _self
                .webview
                .evaluate_script(&format!("window.postMessage({payload}, \"*\");"))?;
            Ok(())
        })
    }

    pub fn send_event<
        F: Fn(&FrameWindowTarget, &mut tao::event_loop::ControlFlow) -> anyhow::Result<()> + Send + 'static,
    >(
        self: &Arc<Self>,
        f: F,
    ) -> anyhow::Result<()> {
        self.event_loop_proxy
            .send_event(utils::UserEvent::FrameEvent(FrameEvent::new(f)))
            .map_err(|_| anyhow::anyhow!("Failed to send event"))
    }

    pub fn send_ipc_event<E: Into<String>, P: serde::Serialize>(
        self: &Arc<Self>,
        event: E,
        payload: P,
    ) -> anyhow::Result<()> {
        let event: String = event.into();
        let payload = serde_json::to_string(&payload)?;
        let _self = self.clone();
        self.send_event(move |_, _| {
            _self
                .webview
                .evaluate_script(&format!("PyFrame.__emit__(\"{event}\", {payload})"))?;
            Ok(())
        })
    }

    pub fn send_ipc_callback<D: serde::Serialize + std::fmt::Debug>(self: &Arc<Self>, data: D) -> anyhow::Result<()> {
        self.send_ipc_event("ipc.callback", serde_json::json!(data))?;
        Ok(())
    }
    #[cfg(target_os = "macos")]
    pub fn switch_menu(self: &Arc<Self>) {}
}
