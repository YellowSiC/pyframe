// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::api_manager::ApiManager;
use anyhow::Result;
use once_cell::unsync::OnceCell;
use pyframe_macros::pyframe_api;
use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::BufReader};

// Globale Audio-Instanz (nur Hauptthread!)
thread_local! {
static AUDIO: OnceCell<(OutputStream, Sink)> = const { OnceCell::new() };
}

pub fn register_api_instances(_api_manager: &mut ApiManager) {
    _api_manager.register_async_api("audio.play_sound", play_sound);
    _api_manager.register_async_api("audio.stop_sound", stop_sound);
}

#[pyframe_api]
fn play_sound(file_path: String) -> Result<()> {
    // OutputStream und Sink initialisieren
    let (stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    // Datei öffnen und dekodieren
    let file = File::open(file_path)?;
    let decoder = Decoder::new(BufReader::new(file))?;

    sink.append(decoder);

    // In OnceCell speichern
    AUDIO.with(|cell| {
        cell.set((stream, sink)).ok(); // nur einmal setzen (wird ignoriert, falls schon gesetzt)
    });

    Ok(())
}

#[pyframe_api]
fn stop_sound() -> Result<()> {
    AUDIO.with(|cell| {
        if let Some((_stream, sink)) = cell.get() {
            sink.stop();
        }
    });
    Ok(())
}
