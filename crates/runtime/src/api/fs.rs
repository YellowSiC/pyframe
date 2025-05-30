// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use anyhow::Result;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use glob::Pattern;
use pyframe_macros::pyframe_api;
use serde::Deserialize;
use serde_json::Value;
use std::{io::Write, path::Path, time::UNIX_EPOCH};

use crate::api_manager::ApiManager;

pub fn register_api_instances(_api_manager: &mut ApiManager) {
    _api_manager.register_async_api("fs.stat", stat);
    _api_manager.register_async_api("fs.exists", exists);
    _api_manager.register_async_api("fs.read", read);
    _api_manager.register_async_api("fs.write", write);
    _api_manager.register_async_api("fs.append", append);
    _api_manager.register_async_api("fs.copy", copy);
    _api_manager.register_async_api("fs.move", move_);
    _api_manager.register_async_api("fs.remove", remove);
    _api_manager.register_async_api("fs.createDir", create_dir);
    _api_manager.register_async_api("fs.createDirAll", create_dir_all);
    _api_manager.register_async_api("fs.readDir", read_dir);
    _api_manager.register_async_api("fs.readDirAll", read_dir_all);
}

#[pyframe_api]
fn stat(path: String) -> Result<Value> {
    let meta = std::fs::metadata(&path)?;

    let data = serde_json::json!({
        "path": path,
        "isDir": meta.is_dir(),
        "isFile": meta.is_file(),
        "isSymlink": meta.file_type().is_symlink(),
        "size": meta.len(),
        "modified": meta.modified().ok().and_then(|m| m.duration_since(UNIX_EPOCH).ok()).map(|d| d.as_millis()),
        "accessed": meta.accessed().ok().and_then(|a| a.duration_since(UNIX_EPOCH).ok()).map(|d| d.as_millis()),
        "created": meta.created().ok().and_then(|c| c.duration_since(UNIX_EPOCH).ok()).map(|d| d.as_millis()),
    });

    Ok(data)
}

#[pyframe_api]
fn exists(path: String) -> Result<bool> {
    let path = std::path::Path::new(&path);
    Ok(path.exists())
}

#[derive(Deserialize)]
enum EncodeType {
    #[serde(rename = "utf8")]
    UTF8,
    #[serde(rename = "base64")]
    BASE64,
}

#[pyframe_api]
fn read(path: String, encode: Option<EncodeType>) -> Result<String> {
    let encode = encode.unwrap_or(EncodeType::UTF8);
    let content = match encode {
        EncodeType::UTF8 => std::fs::read_to_string(path)?,
        EncodeType::BASE64 => {
            let content = std::fs::read(path)?;
            STANDARD.encode(content) // <-- Hier geändert!
        }
    };

    Ok(content)
}

#[pyframe_api]
fn write(path: String, content: String, encode: Option<EncodeType>) -> Result<()> {
    let encode = encode.unwrap_or(EncodeType::UTF8);

    match encode {
        EncodeType::BASE64 => {
            let content = STANDARD.decode(content)?; // <-- HIER
            std::fs::write(path, content)?
        }
        EncodeType::UTF8 => std::fs::write(path, content)?,
    };
    Ok(())
}

#[pyframe_api]
fn append(path: String, content: String, encode: Option<EncodeType>) -> Result<()> {
    let encode = encode.unwrap_or(EncodeType::UTF8);

    match encode {
        EncodeType::BASE64 => {
            let content = STANDARD.decode(content)?;
            std::fs::OpenOptions::new()
                .append(true) // <- hier `.write(true)` entfernt
                .open(path)?
                .write_all(&content)?;
        }
        EncodeType::UTF8 => {
            std::fs::OpenOptions::new()
                .append(true) // <- auch hier `.write(true)` entfernt
                .open(path)?
                .write_all(content.as_bytes())?;
        }
    };

    Ok(())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CopyOptions {
    pub overwrite: Option<bool>,
    pub skip_exist: Option<bool>,
    pub buffer_size: Option<usize>,
    pub copy_inside: Option<bool>,
    pub content_only: Option<bool>,
    pub depth: Option<u64>,
}

fn _create_file_copy_options(options: Option<CopyOptions>) -> fs_extra::file::CopyOptions {
    match options {
        Some(options) => fs_extra::file::CopyOptions {
            overwrite: options.overwrite.unwrap_or(false),
            skip_exist: options.skip_exist.unwrap_or(false),
            buffer_size: options.buffer_size.unwrap_or(64000),
        },
        None => fs_extra::file::CopyOptions::default(),
    }
}

fn _create_dir_copy_options(options: Option<CopyOptions>) -> fs_extra::dir::CopyOptions {
    match options {
        Some(options) => fs_extra::dir::CopyOptions {
            overwrite: options.overwrite.unwrap_or(false),
            skip_exist: options.skip_exist.unwrap_or(false),
            buffer_size: options.buffer_size.unwrap_or(64000),
            copy_inside: options.copy_inside.unwrap_or(false),
            content_only: options.content_only.unwrap_or(false),
            depth: options.depth.unwrap_or(0),
        },
        None => fs_extra::dir::CopyOptions::default(),
    }
}

#[pyframe_api]
fn move_(from: String, to: String, options: Option<CopyOptions>) -> Result<()> {
    let from = std::path::Path::new(&from);

    if from.is_dir() {
        use fs_extra::dir;
        let options = _create_dir_copy_options(options);
        dir::move_dir(from, to, &options)?;
    } else {
        use fs_extra::file;
        let options = _create_file_copy_options(options);
        file::move_file(from, to, &options)?;
    }
    Ok(())
}

#[pyframe_api]
fn copy(from: String, to: String, options: Option<CopyOptions>) -> Result<()> {
    let from = std::path::Path::new(&from);

    if from.is_dir() {
        use fs_extra::dir;
        let options = _create_dir_copy_options(options);
        dir::copy(from, to, &options)?;
    } else {
        use fs_extra::file;
        let options = _create_file_copy_options(options);
        file::copy(from, to, &options)?;
    }

    Ok(())
}

#[pyframe_api]
fn remove(path: String) -> Result<()> {
    let path = std::path::Path::new(&path);

    if path.is_dir() {
        fs_extra::dir::remove(path)?
    } else {
        fs_extra::file::remove(path)?
    };

    Ok(())
}

#[pyframe_api]
fn create_dir(path: String) -> Result<()> {
    std::fs::create_dir(path)?;
    Ok(())
}

#[pyframe_api]
fn create_dir_all(path: String) -> Result<()> {
    std::fs::create_dir_all(path)?;
    Ok(())
}

#[pyframe_api]
fn read_dir(path: Option<String>) -> Result<Vec<String>> {
    let path = path.unwrap_or(".".to_string());

    let mut entries = Vec::new();
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        entries.push(file_name);
    }
    Ok(entries)
}

fn _visit_dirs(dir: &Path, prefix: &Path, files: &mut Vec<String>, excludes: &Vec<Pattern>) -> Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let rel_path = path.strip_prefix(prefix).unwrap();

        // 检查是否需要排除当前文件或文件夹
        let skip_entry = excludes.iter().any(|pattern| {
            let path_str = path.to_str().unwrap();
            pattern.matches(path_str)
        });

        if skip_entry {
            continue;
        }

        if path.is_dir() {
            _visit_dirs(&path, prefix, files, excludes)?;
        } else {
            files.push(rel_path.to_str().unwrap().to_string());
        }
    }

    Ok(())
}

#[pyframe_api]
fn read_dir_all(path: String, excludes: Option<Vec<String>>) -> Result<Vec<String>> {
    let path = Path::new(&path);
    let mut files: Vec<String> = Vec::new();

    let mut exclude_patterns: Vec<Pattern> = vec![];
    if let Some(excludes) = excludes {
        for exclude in excludes {
            let pattern = Pattern::new(&exclude)?;
            exclude_patterns.push(pattern);
        }
    }

    _visit_dirs(path, path, &mut files, &exclude_patterns)?;
    Ok(files)
}
