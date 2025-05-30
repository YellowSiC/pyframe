// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::CoreApplication;
use crate::{unsafe_impl_sync_send, utils::FrameEventLoop};

use super::utils::{arc_mut, ArcMut, IdCounter};
use anyhow::{anyhow, Result};
use global_hotkey::hotkey::HotKey;
use global_hotkey::GlobalHotKeyManager;
use std::collections::HashMap;

pub type PyFrameShortcutsOptions = Vec<crate::options::FrameShortcutOption>;

unsafe_impl_sync_send!(PyFrameShortcutManager);
pub struct PyFrameShortcutManager {
    app: Option<std::sync::Arc<CoreApplication>>,
    manager: GlobalHotKeyManager,
    shortcuts: HashMap<u8, (u8, String, HotKey)>,
    id_counter: IdCounter,
}

impl PyFrameShortcutManager {
    pub fn new(_event_loop: &FrameEventLoop) -> ArcMut<PyFrameShortcutManager> {
        let hotkey_manager = GlobalHotKeyManager::new().unwrap();
        let manager = PyFrameShortcutManager {
            app: None,
            manager: hotkey_manager,
            shortcuts: HashMap::new(),
            id_counter: IdCounter::new(),
        };
        arc_mut(manager)
    }

    #[allow(dead_code)]
    pub fn bind_app(&mut self, app: std::sync::Arc<CoreApplication>) {
        self.app = Some(app);
    }
    pub fn get(&self, id: u8) -> Result<&(u8, String, HotKey)> {
        self.shortcuts
            .get(&id)
            .ok_or(anyhow!("Shortcut with id {} not found", id))
    }

    pub fn register_with_options(&mut self, window_id: u8, options: &PyFrameShortcutsOptions) -> Result<()> {
        for crate::options::FrameShortcutOption {
            modifier,
            key,
            accelerator_str,
            id,
        } in options
        {
            self.register_with_id(window_id, *id, modifier.clone(), key.clone(), accelerator_str.clone())?;
        }
        Ok(())
    }

    pub fn register_with_id(
        &mut self,
        window_id: u8,
        id: u8,
        modifier: Option<crate::hylper::AcceleratorModifier>,
        key: crate::hylper::AcceleratorCode,
        accelerator_str: String,
    ) -> Result<()> {
        if self.shortcuts.contains_key(&id) {
            return Err(anyhow!("Shortcut with id {} already registered", id));
        }

        // Wandelt Option<AcceleratorModifier> in muda::accelerator::Modifiers um
        let modifiers = modifier
            .map(|m| m.into())
            .unwrap_or_else(muda::accelerator::Modifiers::empty);

        let shortcut = HotKey::new(Some(modifiers), key.into());
        self.manager.register(shortcut)?;

        self.shortcuts.insert(id, (window_id, accelerator_str, shortcut));
        Ok(())
    }

    pub fn register(
        &mut self,
        window_id: u8,
        modifier: Option<crate::hylper::AcceleratorModifier>,
        key: crate::hylper::AcceleratorCode,
        accelerator_str: String,
    ) -> Result<u8> {
        let id = self.id_counter.next(&self.shortcuts)?;
        self.register_with_id(window_id, id, modifier, key, accelerator_str)?;
        Ok(id)
    }

    pub fn unregister(&mut self, window_id: u8, id: u8) -> Result<()> {
        let (owner_id, _, _) = self
            .shortcuts
            .get(&id)
            .ok_or(anyhow!("Shortcut with id {} not found", id))?;
        if window_id != *owner_id {
            return Err(anyhow!(
                "Shortcut with id {} can only unregister in window {}",
                id,
                owner_id
            ));
        }
        let (_, _, shortcut) = self
            .shortcuts
            .remove(&id)
            .ok_or(anyhow!("Shortcut with id {} not found", id))?;
        self.manager.unregister(shortcut)?;
        Ok(())
    }

    pub fn unregister_all(&mut self, window_id: u8) -> Result<()> {
        let shortcuts = self
            .shortcuts
            .iter()
            .filter(|(_, (owner_id, _, _))| *owner_id == window_id)
            .map(|(id, _)| *id)
            .collect::<Vec<_>>();
        for id in shortcuts {
            self.unregister(window_id, id)?;
        }
        Ok(())
    }

    pub fn list(&self, window_id: u8) -> Result<Vec<(u8, String)>> {
        Ok(self
            .shortcuts
            .iter()
            .filter(|(_, (owner_id, _, _))| *owner_id == window_id)
            .map(|(id, (_, accelerator_str, _))| (*id, accelerator_str.clone()))
            .collect())
    }
}
