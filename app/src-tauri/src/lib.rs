//! Library entry point for the OpenKotoba desktop app.
//!
//! The GBA core is single threaded, so it lives on its own thread
//! ([`emulator_thread`]) and the UI drives it through commands. Frames are
//! streamed to the frontend over a `tauri::ipc::Channel`.

mod emulator_host;
mod emulator_thread;

use std::path::PathBuf;

use serde::Serialize;
use tauri::ipc::Channel;
use tauri::{AppHandle, Manager, State};

use emulator_host::InputEvent;
use emulator_thread::{Command, EmulatorHandle};

/// Everything the UI needs to know about a freshly loaded ROM.
#[derive(Debug, Clone, Serialize)]
pub struct RomSummary {
    /// Lowercase hex SHA-256 of the ROM, used as the save-file name.
    pub sha256: String,
    /// Game title from the cartridge header.
    pub title: String,
    /// Four character game code, e.g. `BPEE`.
    pub game_code: String,
    /// `ja`, `en`, or `null` when the region is not a supported language.
    pub lang: Option<String>,
    /// Human readable file name.
    pub file_name: String,
}

/// Status line shown in the UI.
#[derive(Debug, Clone, Serialize)]
pub struct RuntimeStatus {
    /// True while a ROM is loaded and frames are being produced.
    pub running: bool,
    /// True when a usable BIOS was found on disk.
    pub bios_found: bool,
    /// Measured frames per second, updated once per second.
    pub fps: f64,
}

/// Returns a short greeting so the UI can prove IPC works.
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Halo, {name}. Selamat belajar!")
}

/// Loads a ROM from disk and starts streaming frames to `on_frame`.
#[tauri::command]
fn load_rom(
    path: String,
    on_frame: Channel<Vec<u8>>,
    app: AppHandle,
    emulator: State<'_, EmulatorHandle>,
) -> Result<RomSummary, String> {
    let bytes = std::fs::read(&path).map_err(|e| format!("cannot read {path}: {e}"))?;

    let info = oemu_emu::parse_rom(&bytes).map_err(|e| format!("not a valid GBA ROM: {e}"))?;
    let file_name = PathBuf::from(&path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.clone());

    let saves_dir = app_dir(&app, "saves")?;
    let sha_hex = to_hex(&info.sha256);

    emulator.ask(|reply| Command::Start {
        rom: bytes,
        sha_hex: sha_hex.clone(),
        saves_dir,
        frames: on_frame,
        reply,
    })??;

    Ok(RomSummary {
        sha256: sha_hex,
        title: info.title,
        game_code: info.game_code,
        lang: info.lang.map(|l| l.code().to_string()),
        file_name,
    })
}

/// Feeds button changes from the keyboard into the emulator.
#[tauri::command]
fn set_keys(events: Vec<InputEvent>, emulator: State<'_, EmulatorHandle>) {
    emulator.send(Command::Input(events));
}

/// Saves the machine state and returns the file path.
#[tauri::command]
fn save_state(app: AppHandle, emulator: State<'_, EmulatorHandle>) -> Result<String, String> {
    let dir = app_dir(&app, "states")?;
    emulator.ask(|reply| Command::SaveState { dir, reply })?
}

/// Loads the state written by [`save_state`].
#[tauri::command]
fn load_state(app: AppHandle, emulator: State<'_, EmulatorHandle>) -> Result<(), String> {
    let dir = app_dir(&app, "states")?;
    emulator.ask(|reply| Command::LoadState { dir, reply })?
}

/// Reports whether a ROM is loaded and whether a BIOS was found.
#[tauri::command]
fn runtime_status(emulator: State<'_, EmulatorHandle>) -> Result<RuntimeStatus, String> {
    let (running, bios_found, fps) = emulator.ask(|reply| Command::Status { reply })?;
    Ok(RuntimeStatus {
        running,
        bios_found,
        fps,
    })
}

/// Persists the battery save before the window closes.
#[tauri::command]
fn stop_emulator(emulator: State<'_, EmulatorHandle>) {
    emulator.send(Command::Flush);
}

/// `<appdata>/open-emulingo/<sub>`
fn app_dir(app: &AppHandle, sub: &str) -> Result<PathBuf, String> {
    app.path()
        .app_data_dir()
        .map(|d| d.join(sub))
        .map_err(|e| format!("cannot resolve app data dir: {e}"))
}

/// Lowercase hex encoding of a byte slice.
fn to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

/// Builds and runs the desktop application.
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(EmulatorHandle::spawn())
        .invoke_handler(tauri::generate_handler![
            greet,
            load_rom,
            set_keys,
            save_state,
            load_state,
            runtime_status,
            stop_emulator
        ])
        .run(tauri::generate_context!())
        .expect("error while running OpenKotoba");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Hex encoding produces lowercase, zero-padded pairs.
    #[test]
    fn hex_encoding_is_lowercase_and_padded() {
        assert_eq!(to_hex(&[0x00, 0xAB, 0xFF]), "00abff");
    }

    /// A 32-byte SHA-256 always encodes to 64 hex characters.
    #[test]
    fn sha256_hex_is_64_chars() {
        assert_eq!(to_hex(&[0u8; 32]).len(), 64);
    }

    /// Greeting proves the IPC command layer is wired.
    #[test]
    fn greet_mentions_the_name() {
        assert!(greet("Budi").contains("Budi"));
    }
}
