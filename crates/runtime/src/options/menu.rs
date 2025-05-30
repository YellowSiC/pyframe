// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::hylper::{AcceleratorCode, AcceleratorModifier};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuItem {
    pub text: String,
    pub enabled: bool,
    pub modifier: AcceleratorModifier,
    pub key: AcceleratorCode,
    pub command_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckMenuItem {
    pub text: String,
    pub enabled: bool,
    pub checked: bool,
    pub modifier: AcceleratorModifier,
    pub key: AcceleratorCode,
    pub command_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IconMenuItem {
    pub text: String,
    pub enabled: bool,
    pub icon_path: PathBuf,
    pub modifier: AcceleratorModifier,
    pub key: AcceleratorCode,
    pub command_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PredefinedMenuItem {
    pub item_type: String,
    pub text: Option<String>,
    pub metadata: Option<AboutMetadata>,
    pub command_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AboutMetadata {
    pub name: Option<String>,
    pub version: Option<String>,
    pub short_version: Option<String>,
    pub authors: Option<Vec<String>>,
    pub comments: Option<String>,
    pub copyright: Option<String>,
    pub license: Option<String>,
    pub website: Option<String>,
    pub website_label: Option<String>,
    pub credits: Option<String>,
    pub icon: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Submenu {
    pub text: String,
    pub enabled: bool,
    pub menu_items: Option<Vec<MenuItem>>,
    pub check_menu: Option<Vec<CheckMenuItem>>,
    pub icon_menu: Option<Vec<IconMenuItem>>,
    pub predefined_menu: Option<Vec<PredefinedMenuItem>>,
    pub command_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuFrame {
    pub menu_items: Option<Vec<MenuItem>>,
    pub sub_menu: Option<Vec<Submenu>>,
    pub check_menu: Option<Vec<CheckMenuItem>>,
    pub icon_menu: Option<Vec<IconMenuItem>>,
    pub predefined_menu: Option<Vec<PredefinedMenuItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemTray {
    pub title: Option<String>,
    pub icon: Option<PathBuf>,
    pub is_template: Option<bool>,
    pub menu_on_left_click: Option<bool>,
    pub tooltip: Option<String>,
}

impl MenuFrame {
    pub fn has_menu_item(&self) -> bool {
        self.menu_items.is_some()
            || self.sub_menu.is_some()
            || self.check_menu.is_some()
            || self.icon_menu.is_some()
            || self.predefined_menu.is_some()
    }
}
