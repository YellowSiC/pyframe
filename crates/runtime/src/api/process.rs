// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

pub const GIT_BUILD_VERSION: Option<&'static str> = Some("v1.2.3");

use crate::api_manager::ApiManager;
use anyhow::{Ok, Result};
use pyframe_macros::{pyframe_api, pyframe_event_api};
use serde::Deserialize;
use serde_json::{json, Value};
use tao::event_loop::ControlFlow;

pub fn register_api_instances(_api_manager: &mut ApiManager) {
    _api_manager.register_api("process.pid", pid);
    _api_manager.register_api("process.currentDir", current_dir);
    _api_manager.register_api("process.currentExe", current_exe);
    _api_manager.register_api("process.env", env);
    _api_manager.register_api("process.args", args);
    _api_manager.register_api("process.setCurrentDir", set_current_dir);
    _api_manager.register_event_api("process.exit", exit);
    _api_manager.register_api("process.version", version);
    _api_manager.register_async_api("process.exec", exec);
    _api_manager.register_async_api("process.open", open);
}

#[pyframe_api]
fn version() -> Result<String> {
    if let Some(version) = GIT_BUILD_VERSION {
        Ok(version.to_string())
    } else {
        Ok("unknown".to_string())
    }
}

#[pyframe_api]
fn pid() -> Result<u32> {
    Ok(std::process::id())
}

#[pyframe_api]
fn current_dir() -> Result<Value> {
    Ok(json!(std::env::current_dir()?))
}

#[pyframe_api]
fn current_exe() -> Result<Value> {
    Ok(json!(std::env::current_exe()?))
}

#[pyframe_api]
fn env() -> Result<Value> {
    let env = std::env::vars().collect::<std::collections::HashMap<String, String>>();
    Ok(json!(env))
}

#[pyframe_api]
fn args() -> Result<Value> {
    let args = std::env::args().collect::<Vec<String>>();
    Ok(json!(args))
}

#[pyframe_api]
fn set_current_dir(path: String) -> Result<()> {
    std::env::set_current_dir(path)?;
    Ok(())
}

#[pyframe_event_api]
fn exit() -> Result<()> {
    *control_flow = ControlFlow::Exit;
    Ok(())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExecOptions {
    pub env: Option<std::collections::HashMap<String, String>>,
    pub current_dir: Option<String>,
    pub detached: Option<bool>,
}

#[pyframe_api]
fn exec(cmd: String, args: Option<Vec<String>>, options: Option<ExecOptions>) -> Result<Value> {
    let mut cmd = std::process::Command::new(cmd);

    if let Some(args) = args {
        cmd.args(args);
    }

    let mut detached = false;
    if let Some(options) = options {
        if let Some(current_dir) = options.current_dir {
            cmd.current_dir(current_dir);
        }
        if let Some(env) = options.env {
            cmd.envs(env);
        }
        detached = options.detached.unwrap_or(false);
    }

    if detached {
        let child = cmd.spawn()?;
        return Ok(json!(child.id()));
    }

    let output = cmd.output()?;

    Ok(json!({
            "status": output.status.code(),
            "stdout": String::from_utf8(output.stdout)?,
            "stderr": String::from_utf8(output.stderr)?,
    }))
}

#[pyframe_api]
fn open(uri: String) -> Result<()> {
    opener::open(uri)?;
    Ok(())
}

// include!(concat!(env!("OUT_DIR"), "/version.rs"));
/*


use std::{env, fs::File, io::Write, path::Path};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let git_version = get_git_version().unwrap_or_else(|| "unknown".to_string());

    let dest_path = Path::new(&out_dir).join("version.rs");
    let mut f = File::create(&dest_path).unwrap();
    writeln!(f, "pub const GIT_BUILD_VERSION: Option<&'static str> = Some(\"{}\");", git_version).unwrap();
}

fn get_git_version() -> Option<String> {
    use std::process::Command;
    let output = Command::new("git")
        .args(&["describe", "--tags", "--always"])
        .output()
        .ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

Cargo.toml


[package]
build = "build.rs"



*/
