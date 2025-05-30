use pyo3::prelude::*;

#[pyfunction]
fn create_webview(config_json: String) -> PyResult<()> {
    env_logger::init();
    let menu_bar = muda::Menu::new();
    let mut event_loop = runtime::utils::FrameEventLoopBuilder::with_user_event();
    // setup accelerator handler on Windows
    #[cfg(target_os = "windows")]
    {
        use tao::platform::windows::EventLoopBuilderExtWindows;
        let menu_bar = menu_bar.clone();
        event_loop.with_msg_hook(move |msg| {
            let translated = unsafe {
                let msg = msg as *const windows_sys::Win32::UI::WindowsAndMessaging::MSG;
                windows_sys::Win32::UI::WindowsAndMessaging::TranslateAcceleratorW(
                    (*msg).hwnd,
                    menu_bar.haccel() as _,
                    msg,
                )
            };
            translated == 1
        });
    }
    let mut event_loop = event_loop.build();
    // Manuell konvertieren
    let app = runtime::CoreApplication::new(&mut event_loop, menu_bar, config_json)
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Init failed: {e}")))?;

    app.run(event_loop)
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Run failed: {e}")))?;

    Ok(())
}

#[pyfunction]
fn create_ico(source_path: &str, target_path: &str) -> PyResult<()> {
    // Convert to Path
    let source_path = std::path::Path::new(source_path);
    let target_path = std::path::Path::new(target_path);

    // Call the core implementation
    icon_creator::create_ico_impl(source_path, target_path)
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("ICO creation failed: {e}")))?;

    Ok(())
}

pub fn get_pyframe_version() -> &'static str {
    // Mapping Cargo versioning (e.g., "1.0-alpha1") to Python's PEP 440 format (e.g., "1.0.0a1")
    // This conversion is a simplified compatibility adjustment and covers most common cases.

    static PYFRAME_VERSION: std::sync::OnceLock<String> = std::sync::OnceLock::new();

    PYFRAME_VERSION.get_or_init(|| env!("CARGO_PKG_VERSION").replace("-alpha", "a").replace("-beta", "b"))
}

#[pymodule]
fn _pyframe(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", get_pyframe_version())?;
    m.add_function(wrap_pyfunction!(create_ico, m)?)?;
    m.add_function(wrap_pyfunction!(create_webview, m)?)?;
    Ok(())
}
