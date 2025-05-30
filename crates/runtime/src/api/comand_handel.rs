use crate::api_manager::ApiManager;
use anyhow::{Ok, Result};
use pyframe_macros::pyframe_api;
use serde_json::Value;

pub fn register_api_instances(_api_manager: &mut ApiManager) {
    _api_manager.register_api("window.menu_comand_handel", menu_comand_handel);
    //_api_manager.register_async_api("audio.stop_sound", stop_sound);
}

#[pyframe_api]
fn menu_comand_handel(data: Value) -> Result<Value> {
    Ok(data)
}
