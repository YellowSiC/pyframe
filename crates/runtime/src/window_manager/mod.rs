// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

pub mod builder;
pub mod protocol;
pub mod window;
use crate::{
    //lock,
    options::window::WindowConfig,
    unsafe_impl_sync_send,
    utils::{arc_mut, ArcMut, FrameWindowTarget, IdCounter},
    CoreApplication,
};
use anyhow::{anyhow, Result};
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tao::window::WindowId;
use window::FrameWindow;
use wry::WebContext;

unsafe_impl_sync_send!(WindowManager);
#[allow(dead_code)]
pub struct WindowManager {
    app: Option<Arc<CoreApplication>>,
    id_counter: IdCounter,
    web_context: WebContext,
    windows: HashMap<u8, Arc<FrameWindow>>,
    id_map: HashMap<WindowId, u8>,
}

impl WindowManager {
    #[allow(dead_code)]
    pub fn new(data_directory: Option<PathBuf>) -> ArcMut<Self> {
        arc_mut(Self {
            app: None,
            id_counter: IdCounter::new(),
            web_context: WebContext::new(data_directory),
            windows: HashMap::new(),
            id_map: HashMap::new(),
        })
    }
    #[allow(dead_code)]
    pub fn bind_app(&mut self, frame: Arc<CoreApplication>) {
        self.app = Some(frame);
    }

    #[allow(dead_code)]
    pub fn open_window(&mut self, target: &FrameWindowTarget, options: &WindowConfig) -> Result<Arc<FrameWindow>> {
        let id = self.id_counter.next(&self.windows)?;

        let frame = self.app.clone().ok_or(anyhow!("Frame not found"))?;

        let frame_window = FrameWindow::new(frame, id, target, options.clone(), self)?;

        self.id_map.insert(frame_window.window_id, frame_window.id);
        self.windows.insert(frame_window.id, frame_window.clone());

        Ok(frame_window)
    }

    #[allow(dead_code)]
    pub fn get_first_window_id(&self) -> Result<WindowId> {
        let first = self
            .windows
            .values()
            .next()
            .ok_or_else(|| anyhow!("No windows available"))?;
        Ok(first.window_id)
    }
    pub fn get_first_window(&self) -> Result<Arc<FrameWindow>> {
        self.windows
            .values()
            .next()
            .cloned()
            .ok_or_else(|| anyhow!("No windows available"))
    }

    #[allow(dead_code)]
    pub fn get_window(&self, id: u8) -> Result<Arc<FrameWindow>> {
        self.windows.get(&id).cloned().ok_or(anyhow!("Window {id} not found"))
    }
    #[allow(dead_code)]
    pub fn get_window_inner(&self, window_id: WindowId) -> Result<Arc<FrameWindow>> {
        let id = self
            .id_map
            .get(&window_id)
            .cloned()
            .ok_or(anyhow!("Window not found"))?;
        self.get_window(id)
    }
    #[allow(dead_code)]
    pub fn close_window(&mut self, id: u8) -> Result<()> {
        let niva_window = self.windows.remove(&id).ok_or(anyhow!("Window {id} not found"))?;
        self.id_map
            .remove(&niva_window.window_id)
            .ok_or(anyhow!("Window {id} not found"))?;

        let _frame = self.app.clone().ok_or(anyhow!("Frame not found"))?;
        _frame.shortcut()?.unregister_all(id)?;
        // frame.tray()?.destroy_all(id)?;
        Ok(())
    }
    #[allow(dead_code)]
    pub fn list_windows(&self) -> Vec<&Arc<FrameWindow>> {
        self.windows.values().collect()
    }
    #[allow(dead_code)]
    pub fn close_window_inner(&mut self, window_id: WindowId) -> Result<()> {
        let id = self.id_map.get(&window_id).ok_or(anyhow!("Window not found"))?;
        self.close_window(*id)
    }
}
