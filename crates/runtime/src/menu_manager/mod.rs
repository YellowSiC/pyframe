use crate::hylper::{
    create_check_menuitem, create_icon_menuitem, create_menuitem, create_predefined_menu_item, create_submenu,
};
use crate::options::menu::MenuFrame;
use crate::utils::ArcMut;
use crate::utils::{arc_mut, IdCounter};
use crate::{unsafe_impl_sync_send, CoreApplication};
use anyhow::{anyhow, Result};
use muda::{Menu, MenuItemKind};
use std::collections::HashMap;

unsafe_impl_sync_send!(PyFrameMenuManager);
#[allow(dead_code)]
pub struct PyFrameMenuManager {
    app: Option<std::sync::Arc<CoreApplication>>,
    manager: Menu,
    pub items: HashMap<muda::MenuId, (MenuItemKind, Option<String>)>,
    id_counter: IdCounter, // Falls noch benötigt für andere Zwecke
}

impl PyFrameMenuManager {
    pub fn new(menu: muda::Menu) -> ArcMut<PyFrameMenuManager> {
        let manager = PyFrameMenuManager {
            app: None,
            manager: menu,
            items: HashMap::new(),
            id_counter: IdCounter::new(),
        };
        arc_mut(manager)
    }
    #[allow(dead_code)]
    pub fn bind_app(&mut self, app: std::sync::Arc<CoreApplication>) {
        self.app = Some(app);
    }
    pub fn get(&self, id: &muda::MenuId) -> Result<&(MenuItemKind, Option<String>)> {
        self.items.get(id).ok_or(anyhow!("MenuItem with id {:?} not found", id))
    }

    pub fn register_with_options(&mut self, options: MenuFrame) -> Result<()> {
        self.register_menu_items(options)
    }

    pub fn register_menu_items(&mut self, menu_system: MenuFrame) -> Result<()> {
        if let Some(menu_items) = menu_system.menu_items {
            for item in menu_items {
                let (menu_id, menu_item, command_id) = create_menuitem(item);
                self.manager.append(&menu_item)?;
                self.items
                    .insert(menu_id, (MenuItemKind::MenuItem(menu_item), command_id));
            }
        }

        // Füge Submenüs hinzu
        if let Some(sub_menus) = &menu_system.sub_menu {
            for submenu in sub_menus {
                self.manager
                    .append(&create_submenu(submenu.clone(), &mut self.items))
                    .ok();
            }
        }
        if let Some(menu_items) = menu_system.check_menu {
            for item in menu_items {
                let (menu_id, menu_item, command_id) = create_check_menuitem(item);
                self.manager.append(&menu_item)?;
                self.items.insert(menu_id, (MenuItemKind::Check(menu_item), command_id));
            }
        }

        if let Some(menu_items) = menu_system.icon_menu {
            for item in menu_items {
                let (menu_id, menu_item, command_id) = create_icon_menuitem(item);
                self.manager.append(&menu_item)?;
                self.items.insert(menu_id, (MenuItemKind::Icon(menu_item), command_id));
            }
        }

        if let Some(menu_items) = menu_system.predefined_menu {
            for item in menu_items {
                let (menu_id, menu_item, command_id) = create_predefined_menu_item(item);
                self.manager.append(&menu_item)?;
                self.items
                    .insert(menu_id, (MenuItemKind::Predefined(menu_item), command_id));
            }
        }

        Ok(())
    }

    pub fn unregister(&mut self, menu_id: &muda::MenuId) -> Result<()> {
        let (menu_kind, _) = self
            .items
            .get(menu_id)
            .ok_or(anyhow!("menuitem with id {:?} not found", menu_id))?;

        let menu_item_ref: &dyn muda::IsMenuItem = match menu_kind {
            MenuItemKind::MenuItem(item) => item,
            MenuItemKind::Check(item) => item,
            MenuItemKind::Icon(item) => item,
            MenuItemKind::Predefined(item) => item,
            MenuItemKind::Submenu(item) => item,
        };

        self.manager.remove(menu_item_ref)?;
        self.items.remove(menu_id);

        Ok(())
    }

    pub fn unregister_all(&mut self) -> Result<()> {
        let menu_ids = self.items.keys().cloned().collect::<Vec<_>>();
        for menu_id in menu_ids {
            self.unregister(&menu_id)?;
        }
        Ok(())
    }

    pub fn get_mutable_menu_manager(&mut self) -> Result<&mut Menu> {
        Ok(&mut self.manager)
    }

    pub fn get_menu_manager(&self) -> Result<Menu> {
        Ok(self.manager.clone())
    }
}
