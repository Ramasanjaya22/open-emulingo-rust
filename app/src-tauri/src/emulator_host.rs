//! Owns the running emulator instance and all of its side data.
//!
//! The host is the only thing that touches the [`oemu_emu::Emulator`] trait from
//! the app. It handles battery saves, save states, frame throughput counting and
//! translates the UI's input events into key presses.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use oemu_emu::adapter_rustboyadvance::CoreEmulator;
use oemu_emu::{Emulator, KeyState};
use serde::Deserialize;

/// How long a save state file keeps its extension; `1` is the only slot for now.
const STATE_EXT: &str = "ss1";

/// How often the battery save is flushed to disk while playing.
const BATTERY_FLUSH_INTERVAL: Duration = Duration::from_secs(30);

/// One button transition coming from the UI.
#[derive(Debug, Clone, Deserialize)]
pub struct InputEvent {
    /// Key name: `A`, `B`, `Start`, `Select`, `Up`, `Down`, `Left`, `Right`, `L`, `R`.
    pub key: String,
    /// True when the key went down, false when it came up.
    pub pressed: bool,
}

impl InputEvent {
    /// Maps the UI's key name onto the emulator's key enum.
    fn to_key(&self) -> Option<KeyState> {
        match self.key.as_str() {
            "A" => Some(KeyState::A),
            "B" => Some(KeyState::B),
            "Start" => Some(KeyState::Start),
            "Select" => Some(KeyState::Select),
            "Up" => Some(KeyState::Up),
            "Down" => Some(KeyState::Down),
            "Left" => Some(KeyState::Left),
            "Right" => Some(KeyState::Right),
            "L" => Some(KeyState::L),
            "R" => Some(KeyState::R),
            _ => None,
        }
    }
}

/// A loaded emulator plus its on-disk save locations.
pub struct EmulatorHost {
    emulator: CoreEmulator,
    loaded: bool,
    bios_found: bool,
    sha_hex: String,
    save_path: Option<PathBuf>,
    state_path: Option<PathBuf>,
    last_flush: Instant,
    frames: u32,
    window_start: Instant,
    fps: f64,
}

impl EmulatorHost {
    /// Creates a host with a BIOS discovered from disk (if any).
    pub fn new() -> Self {
        // Probe for a BIOS once so `runtime_status` can report it to the UI.
        let bios_found = oemu_emu::adapter_rustboyadvance::find_bios().is_some();
        Self {
            emulator: CoreEmulator::with_discovered_bios(),
            loaded: false,
            bios_found,
            sha_hex: String::new(),
            save_path: None,
            state_path: None,
            last_flush: Instant::now(),
            frames: 0,
            window_start: Instant::now(),
            fps: 0.0,
        }
    }

    /// True once a ROM has been loaded successfully.
    pub fn has_rom(&self) -> bool {
        self.loaded
    }

    /// True when a 16 KiB BIOS was found on disk.
    pub fn bios_found(&self) -> bool {
        self.bios_found
    }

    /// The most recently measured frame rate.
    pub fn fps(&self) -> f64 {
        self.fps
    }

    /// Loads ROM bytes, restores any battery save, and primes the first frames.
    pub fn start(&mut self, rom: Vec<u8>, sha_hex: &str, saves_dir: &Path) -> Result<(), String> {
        std::fs::create_dir_all(saves_dir).map_err(|e| format!("cannot create saves dir: {e}"))?;

        self.emulator = CoreEmulator::with_discovered_bios();
        self.emulator
            .load_rom(&rom)
            .map_err(|e| format!("emulator rejected the ROM: {e}"))?;

        self.sha_hex = sha_hex.to_string();
        self.save_path = Some(saves_dir.join(format!("{sha_hex}.sav")));

        if let Some(path) = &self.save_path {
            if let Ok(bytes) = std::fs::read(path) {
                self.emulator.load_battery(&bytes);
            }
        }

        self.loaded = true;
        self.last_flush = Instant::now();
        self.frames = 0;
        self.window_start = Instant::now();
        Ok(())
    }

    /// Advances one frame and returns the RGBA bytes when a ROM is loaded.
    pub fn step(&mut self) -> Option<Vec<u8>> {
        if !self.loaded {
            return None;
        }

        let frame = self.emulator.run_frame().as_bytes().to_vec();

        self.frames += 1;
        let elapsed = self.window_start.elapsed();
        if elapsed >= Duration::from_secs(1) {
            self.fps = self.frames as f64 / elapsed.as_secs_f64();
            self.frames = 0;
            self.window_start = Instant::now();
        }

        if self.last_flush.elapsed() >= BATTERY_FLUSH_INTERVAL {
            self.flush_battery();
        }

        Some(frame)
    }

    /// Applies a batch of button transitions.
    pub fn apply_input(&mut self, events: &[InputEvent]) {
        for event in events {
            if let Some(key) = event.to_key() {
                self.emulator.set_key(key, event.pressed);
            }
        }
    }

    /// Writes the battery save to disk, if the cartridge has one.
    pub fn flush_battery(&mut self) {
        let (Some(path), Some(bytes)) = (&self.save_path, self.emulator.battery_save()) else {
            return;
        };
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(path, bytes);
        self.last_flush = Instant::now();
    }

    /// Writes a save state and returns the file path.
    pub fn save_state(&mut self, states_dir: &Path) -> Result<String, String> {
        if !self.loaded {
            return Err("no ROM loaded".into());
        }
        std::fs::create_dir_all(states_dir)
            .map_err(|e| format!("cannot create states dir: {e}"))?;

        let path = states_dir.join(format!("{}.{}", self.sha_hex, STATE_EXT));
        std::fs::write(&path, self.emulator.save_state())
            .map_err(|e| format!("cannot write save state: {e}"))?;

        self.state_path = Some(path.clone());
        Ok(path.to_string_lossy().to_string())
    }

    /// Reads the save state written by [`Self::save_state`].
    pub fn load_state(&mut self, states_dir: &Path) -> Result<(), String> {
        if !self.loaded {
            return Err("no ROM loaded".into());
        }
        let path = self
            .state_path
            .clone()
            .unwrap_or_else(|| states_dir.join(format!("{}.{}", self.sha_hex, STATE_EXT)));

        let bytes = std::fs::read(&path)
            .map_err(|e| format!("cannot read save state {}: {e}", path.display()))?;
        self.emulator.load_state(&bytes)
    }
}

impl Default for EmulatorHost {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Unknown key names are ignored instead of panicking.
    #[test]
    fn unknown_key_is_ignored() {
        let event = InputEvent {
            key: "Turbo".into(),
            pressed: true,
        };
        assert!(event.to_key().is_none());
    }

    /// Every key the UI can send maps onto the emulator's key set.
    #[test]
    fn all_ui_keys_map_to_emulator_keys() {
        for name in [
            "A", "B", "Start", "Select", "Up", "Down", "Left", "Right", "L", "R",
        ] {
            let event = InputEvent {
                key: name.into(),
                pressed: true,
            };
            assert!(event.to_key().is_some(), "unmapped key {name}");
        }
    }

    /// Stepping without a ROM yields no frame, so the UI simply idles.
    #[test]
    fn step_is_noop_without_rom() {
        let mut host = EmulatorHost::new();
        assert!(!host.has_rom());
        assert!(host.step().is_none());
    }

    /// Saving a state before a ROM is loaded is an error, not a panic.
    #[test]
    fn save_state_requires_rom() {
        let mut host = EmulatorHost::new();
        let dir = std::env::temp_dir();
        assert!(host.save_state(&dir).is_err());
    }

    /// Full pipeline: load a real ROM, run frames, save and reload a state.
    #[test]
    #[ignore = "needs OEMU_ROM_EN pointing at a real .gba file"]
    fn real_rom_plays_saves_and_restores() {
        let path = std::env::var("OEMU_ROM_EN").expect("set OEMU_ROM_EN to an English .gba file");
        let rom = std::fs::read(&path).expect("read ROM");

        let dir = std::env::temp_dir().join(format!("oemu-e2e-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let saves = dir.join("saves");
        let states = dir.join("states");

        let mut host = EmulatorHost::new();
        host.start(rom, "e2e", &saves).expect("start");
        assert!(host.has_rom());

        // Boot far enough that the title screen is drawn.
        let mut last = None;
        for _ in 0..400 {
            last = host.step();
        }
        let frame = last.expect("a frame was produced");
        assert_eq!(frame.len(), 240 * 160 * 4);

        // A save state round-trips without error.
        let written = host.save_state(&states).expect("save state");
        assert!(std::path::Path::new(&written).exists());
        host.load_state(&states).expect("load state");

        // Pressing and releasing a button is accepted.
        host.apply_input(&[InputEvent {
            key: "Start".into(),
            pressed: true,
        }]);
        host.step();
        host.apply_input(&[InputEvent {
            key: "Start".into(),
            pressed: false,
        }]);

        let _ = std::fs::remove_dir_all(&dir);
    }
}
