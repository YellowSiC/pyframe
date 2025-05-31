from dataclasses import dataclass, field
from pathlib import Path
from typing import Callable, List, Literal, Optional

from pyframe.model.models import (AboutMetadata, AcceleratorCode,
                                  AcceleratorModifier, CheckMenuItem,
                                  IconMenuItem, MenuFrame, MenuItem,
                                  PredefinedMenuItem, Submenu, SystemTray)

PredefinedTyp = Literal[
    "separator",
    "about",
    "close_window",
    "copy",
    "fullscreen",
    "cut",
    "hide",
    "hide_others",
    "maximize",
    "minimize",
    "paste",
    "bring_all_to_front",
    "quit",
    "redo",
    "select_all",
    "show_all",
    "undo",
]


def create_about_metadata(
    name: Optional[str] = None,
    version: Optional[str] = None,
    short_version: Optional[str] = None,
    authors: Optional[List[str]] = None,
    comments: Optional[str] = None,
    copyright: Optional[str] = None,
    license: Optional[str] = None,
    website: Optional[str] = None,
    website_label: Optional[str] = None,
    credits: Optional[str] = None,
    icon: Optional[Path] = None,
) -> AboutMetadata:
    return AboutMetadata(
        name=name,
        version=version,
        short_version=short_version,
        authors=authors,
        comments=comments,
        copyright=copyright,
        license=license,
        website=website,
        website_label=website_label,
        credits=credits,
        icon=icon,
    )


class SubMenu:
    def __init__(self, title: str, enabled: bool):
        self.text: str = title
        self.enabled: bool = enabled
        self.menu_items: List[MenuItem] = []
        self.check_menu: List[CheckMenuItem] = []
        self.icon_menu: List[IconMenuItem] = []
        self.predefined_menu: List[PredefinedMenuItem] = []

    def add_predefined_item(
        self,
        item_type: PredefinedTyp,
        text: Optional[str] = None,
        metadata: Optional[AboutMetadata] = None,
        command: Optional[Callable] = None,
    ) -> "SubMenu":
        self.predefined_menu.append(PredefinedMenuItem(
            item_type=item_type,
            text=text,
            metadata=metadata,
            command=command
        ))
        return self

    def add_menu_item(
        self,
        text: str,
        enabled: bool,
        modifier: AcceleratorModifier,
        key: AcceleratorCode,
        command: Optional[Callable] = None,
    ) -> "SubMenu":
        self.menu_items.append(MenuItem(
            text=text,
            enabled=enabled,
            modifier=modifier,
            key=key,
            command=command
        ))
        return self

    def add_check_item(
        self,
        text: str,
        enabled: bool,
        checked: bool,
        modifier: AcceleratorModifier,
        key: AcceleratorCode,
        command: Optional[Callable] = None,
    ) -> "SubMenu":
        self.check_menu.append(CheckMenuItem(
            text=text,
            enabled=enabled,
            checked=checked,
            modifier=modifier,
            key=key,
            command=command
        ))
        return self

    def add_icon_item(
        self,
        text: str,
        enabled: bool,
        icon_path: Path,
        modifier: AcceleratorModifier,
        key: AcceleratorCode,
        command: Optional[Callable] = None,
    ) -> "SubMenu":
        self.icon_menu.append(IconMenuItem(
            text=text,
            enabled=enabled,
            icon=icon_path,
            modifier=modifier,
            key=key,
            command=command
        ))
        return self

    def build(self) -> Submenu:
        return Submenu(
            text=self.text,
            enabled=self.enabled,
            menu_items=self.menu_items or None,
            check_menu=self.check_menu or None,
            icon_menu=self.icon_menu or None,
            predefined_menu=self.predefined_menu or None,
        )


class Menu:
    def __init__(self):
        self.menu_items: List[MenuItem] = []
        self.sub_items: List[Submenu] = []
        self.check_menu: List[CheckMenuItem] = []
        self.icon_menu: List[IconMenuItem] = []
        self.predefined_menu: List[PredefinedMenuItem] = []
        self.system_tray:Optional[SystemTray]=None

    def add_predefined_item(
        self,
        item_type: PredefinedTyp,
        text: Optional[str] = None,
        metadata: Optional[AboutMetadata] = None,
        command: Optional[Callable] = None,
    ) -> "Menu":
        self.predefined_menu.append(PredefinedMenuItem(
            item_type=item_type,
            text=text,
            metadata=metadata,
            command=command
        ))
        return self

    def add_menu_item(
        self,
        text: str,
        enabled: bool,
        modifier: AcceleratorModifier,
        key: AcceleratorCode,
        command: Optional[Callable] = None,
    ) -> "Menu":
        self.menu_items.append(MenuItem(
            text=text,
            enabled=enabled,
            modifier=modifier,
            key=key,
            command=command
        ))
        return self

    def add_check_item(
        self,
        text: str,
        enabled: bool,
        checked: bool,
        modifier: AcceleratorModifier,
        key: AcceleratorCode,
        command: Optional[Callable] = None,
    ) -> "Menu":
        self.check_menu.append(CheckMenuItem(
            text=text,
            enabled=enabled,
            checked=checked,
            modifier=modifier,
            key=key,
            command=command
        ))
        return self

    def add_icon_item(
        self,
        text: str,
        enabled: bool,
        icon_path: Path,
        modifier: AcceleratorModifier,
        key: AcceleratorCode,
        command: Optional[Callable] = None,
    ) -> "Menu":
        self.icon_menu.append(IconMenuItem(
            text=text,
            enabled=enabled,
            icon=icon_path,
            modifier=modifier,
            key=key,
            command=command
        ))
        return self

    def add_submenu(
        self,
        submenus: Optional[List[SubMenu]] = None,
        command: Optional[Callable] = None,
    ) -> "Menu":
        if submenus:
            for SubMenu_ in submenus:
                self.sub_items.append(SubMenu_.build())
        return self
    

    def add_system_tray(
        self,
        title: Optional[str] = None,
        icon: Optional[Path] = None,
        is_template: Optional[bool] = None,
        menu_on_left_click: Optional[bool] = None,
        tooltip: Optional[str] = None
    ):
        self.system_tray = SystemTray(
            title=title,
            icon=icon,
            is_template=is_template,
            menu_on_left_click=menu_on_left_click,
            tooltip=tooltip,
        )

    def build(self) -> MenuFrame:
        return MenuFrame(
            menu_items=self.menu_items or None,
            sub_menu=self.sub_items or None,
            check_menu=self.check_menu or None,
            icon_menu=self.icon_menu or None,
            predefined_menu=self.predefined_menu or None,
            system_tray=self.system_tray or None
        )
