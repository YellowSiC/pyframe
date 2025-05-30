use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use tray_icon::TrayIcon;

use crate::{
    options::FrameTrayOptions,
    unsafe_impl_sync_send,
    utils::{arc_mut, ArcMut, FrameWindowTarget, IdCounter},
    CoreApplication,
};

unsafe_impl_sync_send!(PyFrameTrayManager);
#[allow(dead_code)]
pub struct PyFrameTrayManager {
    id_counter: IdCounter,
    app: Option<Arc<CoreApplication>>,
    trays: HashMap<u8, (u8, HashSet<u8>, ArcMut<TrayIcon>)>,
}

impl PyFrameTrayManager {
    pub fn new() -> ArcMut<PyFrameTrayManager> {
        arc_mut(PyFrameTrayManager {
            id_counter: IdCounter::new(),
            app: None,
            trays: HashMap::new(),
        })
    }
    pub fn bind_app(&mut self, app: Arc<CoreApplication>) {
        self.app = Some(app);
    }

    pub fn get_window_id_by_menu_id(&self, menu_id: u8) -> Option<u8> {
        for (window_id, menu_ids, _) in self.trays.values() {
            if menu_ids.contains(&menu_id) {
                return Some(*window_id);
            }
        }
        None
    }

    pub fn create(
        &mut self,
        _window_id: u8,
        _options: &FrameTrayOptions,
        _target: &FrameWindowTarget,
    ) -> anyhow::Result<u8> {
        /*         let id = self.id_counter.next(&self.trays)?;
        let tray = self.build_tray(id, window_id, options, target)?;
        let menu_ids = if let Some(options) = &options.menu {
            Self::get_menu_ids(options)
        } else {
            HashSet::new()
        };

        self.trays.insert(id, (window_id, menu_ids, tray)); */
        Ok(2)
    }

    /*     fn get_menu_ids(_options) -> HashSet<u8> {
        Ok(HashSet::new())

    } */
}
