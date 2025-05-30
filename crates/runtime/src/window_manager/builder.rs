// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::sync::Arc;

//use serde_json::json;

use crate::{
    log_if_err, options::window::WindowConfig, set_property, set_property_some, utils::FrameWindowTarget,
    CoreApplication,
};

use super::WindowManager;

pub struct FrameBuilder;

impl FrameBuilder {
    pub fn build_window(
        _app: &Arc<CoreApplication>,
        _manager: &WindowManager,
        _id: u8,
        options: &WindowConfig,
        target: &FrameWindowTarget,
    ) -> anyhow::Result<tao::window::Window> {
        let mut builder = tao::window::WindowBuilder::new();

        set_property!(
            builder,
            with_title,
            options.window_title.clone().unwrap_or("PyFrame".to_string())
        );
        set_property_some!(builder, with_always_on_bottom, options.window_always_on_bottom);
        if let Some(icon_path) = &options.window_window_icon {
            let icon = _app.resource().load_icon(icon_path)?;
            set_property!(builder, with_window_icon, Some(icon));
        } else {
            let icon = _app.resource().load_icon_from_bytes(crate::assets::DEFAULT_LOGO)?;
            set_property!(builder, with_window_icon, Some(icon));
        }
        set_property_some!(builder, with_always_on_top, options.window_always_on_top);
        set_property_some!(builder, with_background_color, options.window_background_color);
        set_property_some!(builder, with_closable, options.window_closable);
        set_property_some!(builder, with_content_protection, options.window_content_protection);
        set_property_some!(builder, with_decorations, options.window_decorations);
        set_property_some!(builder, with_focused, options.window_focused);
        set_property_some!(
            builder,
            with_fullscreen,
            options.window_fullscreen.map(|b| {
                if b {
                    Some(tao::window::Fullscreen::Borderless(None))
                } else {
                    None
                }
            })
        );

        set_property_some!(
            builder,
            with_theme,
            options.window_theme.clone().map(|theme_str| match theme_str.as_str() {
                "dark" => Some(tao::window::Theme::Dark),
                "light" => Some(tao::window::Theme::Light),
                _ => None,
            })
        );

        set_property_some!(builder, with_inner_size, options.window_inner_size);
        set_property_some!(builder, with_max_inner_size, options.window_max_inner_size);
        set_property_some!(builder, with_maximizable, options.window_maximizable);
        set_property_some!(builder, with_maximized, options.window_maximized);
        set_property_some!(builder, with_min_inner_size, options.window_min_inner_size);
        set_property_some!(builder, with_minimizable, options.window_minimizable);
        set_property_some!(builder, with_minimizable, options.window_minimizable);
        set_property_some!(builder, with_minimizable, options.window_minimizable);
        set_property_some!(builder, with_minimizable, options.window_minimizable);
        set_property_some!(builder, with_minimizable, options.window_minimizable);
        set_property_some!(builder, with_position, options.window_position);
        set_property_some!(builder, with_resizable, options.window_resizable);
        set_property_some!(builder, with_transparent, options.window_transparent);
        set_property_some!(builder, with_visible, options.window_visible);
        set_property_some!(
            builder,
            with_visible_on_all_workspaces,
            options.window_visible_on_all_workspaces
        );

        #[cfg(target_os = "macos")]
        if let Some(macos_extra) = &options.macos_extra {
            use tao::platform::macos::{WindowBuilderExtMacOS, WindowExtMacOS};

            if let Some(parent) = &macos_extra.parent_window {
                let parent = _manager.get_window(*parent)?;
                let parent = parent.ns_window();
                set_property!(builder, with_parent_window, parent);
            }

            set_property_some!(
                builder,
                with_movable_by_window_background,
                macos_extra.movable_by_window_background
            );
            set_property_some!(builder, with_titlebar_transparent, macos_extra.titlebar_transparent);
            set_property_some!(builder, with_titlebar_hidden, macos_extra.titlebar_hidden);
            set_property_some!(
                builder,
                with_titlebar_buttons_hidden,
                macos_extra.titlebar_buttons_hidden
            );
            set_property_some!(builder, with_title_hidden, macos_extra.title_hidden);
            set_property_some!(builder, with_fullsize_content_view, macos_extra.fullsize_content_view);
            set_property_some!(builder, with_resize_increments, macos_extra.resize_increments);
            set_property_some!(builder, with_disallow_hidpi, macos_extra.disallow_hidpi);
            set_property_some!(builder, with_has_shadow, macos_extra.has_shadow);
            set_property_some!(
                builder,
                with_automatic_window_tabbing,
                macos_extra.automatic_window_tabbing
            );
            set_property_some!(builder, with_tabbing_identifier, &macos_extra.tabbing_identifier);

            set_property_some!(builder, with_traffic_light_inset, macos_extra.traffic_light_inset);
        }

        #[cfg(target_os = "windows")]
        if let Some(windows_extra) = &options.windows_extra {
            use tao::platform::windows::{WindowBuilderExtWindows, WindowExtWindows};
            use windows::Win32::Foundation::HWND;

            if let Some(parent) = &windows_extra.parent_window {
                let parent = _manager.get_window(*parent)?;
                let parent_hwnd = HWND(parent.hwnd() as _);
                let parent_isize = parent_hwnd.0 as isize;
                set_property!(builder, with_parent_window, parent_isize);
            }

            if let Some(owner) = &windows_extra.parent_window {
                let owner = _manager.get_window(*owner)?;
                let owner_hwnd = HWND(owner.hwnd() as _);
                let owner_isize = owner_hwnd.0 as isize;
                set_property!(builder, with_owner_window, owner_isize);
            }

            if let Some(icon_path) = &windows_extra.taskbar_icon {
                let icon = _app.resource().load_icon(icon_path)?;
                set_property!(builder, with_taskbar_icon, Some(icon));
            }

            set_property_some!(builder, with_skip_taskbar, windows_extra.skip_taskbar);
            set_property_some!(builder, with_undecorated_shadow, windows_extra.undecorated_shadow);
        }
        #[cfg(target_os = "linux")]
        if let Some(linux_extra) = &options.linux_extra {
            use tao::platform::unix::WindowBuilderExtUnix;

            set_property_some!(builder, with_app_paintable, linux_extra.app_paintable);
            set_property_some!(builder, with_skip_taskbar, linux_extra.skip_taskbar);
            set_property_some!(builder, with_cursor_moved_event, linux_extra.cursor_moved_event);
            set_property_some!(builder, with_double_buffered, linux_extra.double_buffered);
            set_property_some!(builder, with_rgba_visual, linux_extra.rgba_visual);
            // set_property_some!(builder, with_transient_for, &linux_extra.transient_for);
            set_property_some!(builder, with_transparent_draw, linux_extra.transparent_draw);
        }
        let window = builder.build(target)?;

        Ok(window)
    }

    pub fn build_webview(
        _app: &Arc<CoreApplication>,
        options: &WindowConfig,
        target: tao::window::Window,
        _web_context: &mut wry::WebContext,
    ) -> anyhow::Result<(tao::window::Window, wry::WebView)> {
        let cloned_app = _app.clone();

        let app_settings = cloned_app.launch_info.options.clone();
        let internal_api = app_settings.internal_api;
        let sock = cloned_app.launch_info.socket_settings.clone();
        let external_proto = cloned_app.launch_info.options.web_proto.clone();

        let mut builder = wry::WebViewBuilder::new().with_initialization_script(crate::assets::INITIALIZE_SCRIPT);

        wry::WebViewBuilder::with_web_context(_web_context);

        if internal_api.unwrap_or(false) {
            builder = builder
                .with_initialization_script(crate::assets::SOCKETIO_SCRIPT)
                .with_initialization_script(crate::assets::INITIALIZE_SCRIPT)
                .with_initialization_script(crate::utils::generate_socketio_js(sock, app_settings))
                .with_initialization_script(crate::assets::INITIALIZEPY_SCRIPT);
        } else {
            builder = builder.with_initialization_script(crate::assets::INITIALIZE_SCRIPT);
        }

        set_property_some!(builder, with_accept_first_mouse, options.webview_accept_first_mouse);
        set_property_some!(builder, with_autoplay, options.webview_autoplay);
        set_property_some!(
            builder,
            with_back_forward_navigation_gestures,
            options.webview_back_forward_navigation_gestures
        );
        set_property_some!(builder, with_background_color, options.webview_background_color);
        set_property_some!(builder, with_clipboard, options.webview_clipboard);
        set_property_some!(builder, with_devtools, options.webview_devtools);
        set_property_some!(builder, with_focused, options.window_focused);
        set_property_some!(builder, with_hotkeys_zoom, options.webview_hotkeys_zoom);
        set_property_some!(builder, with_incognito, options.webview_incognito);
        set_property_some!(builder, with_transparent, options.webview_transparent);
        set_property_some!(builder, with_user_agent, &options.webview_user_agent);
        set_property_some!(builder, with_visible, options.webview_visible);

        if let Some((code, allowed)) = &options.webview_initialization_script_for_main_only {
            builder = builder.with_initialization_script_for_main_only(code, *allowed)
        }
        if options.webview_javascript_disabled.is_some() {
            builder = builder.with_javascript_disabled()
        }

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        if let Some(policy) = options.webview_background_throttling.clone() {
            let wry_policy: wry::BackgroundThrottlingPolicy = policy.into();
            set_property!(builder, with_background_throttling, wry_policy);
        }

        builder = match external_proto {
            Some(proto) if proto.starts_with("https") || proto.starts_with("http") => {
                crate::window_manager::protocol::build_full_url(builder, cloned_app.clone())?
            }
            _ => crate::window_manager::protocol::render_web_protocol(cloned_app.clone(), builder)?,
        };
        let win_id = target.id();

        let ipc_app = _app.clone();

        set_property!(builder, with_ipc_handler, {
            let _ipc_app = cloned_app.clone();
            move |request: wry::http::Request<String>| {
                let window_result = ipc_app.window().and_then(|w| w.get_window_inner(win_id));
                let request_str = request.body();

                match window_result {
                    Ok(window) => {
                        if let Err(err) = ipc_app.api().and_then(|w| w.call(&window, request_str.to_string())) {
                            log_if_err!(window.send_ipc_callback(serde_json::json!({
                                "ipc.error": err.to_string(),
                            })));
                        }
                    }
                    Err(err) => {
                        println!("WARN: Window for id {:?} not found: {:?}", win_id, err);
                    }
                }
            }
        });

        let webview = builder;

        #[cfg(any(target_os = "windows", target_os = "macos", target_os = "ios", target_os = "android"))]
        let webview = webview.build(&target)?;
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "ios", target_os = "android")))]
        let webview = {
            use tao::platform::unix::WindowExtUnix;
            use wry::WebViewBuilderExtUnix;
            let vbox = target.default_vbox().unwrap();
            webview.build_gtk(vbox)?
        };

        Ok((target, webview))
    }
}
