//! Adapter between oemu's [`Emulator`] trait and `rustboyadvance-core`.
//!
//! This is the only file allowed to name core types (ADR 0002). Everything is
//! converted at the boundary into oemu's own `Frame` / `KeyState` values.

use rustboyadvance_core::cartridge::GamepakBuilder;
use rustboyadvance_core::gba::GameBoyAdvance;
use rustboyadvance_core::sound::interface::{DynAudioInterface, SimpleAudioInterface};
use rustboyadvance_utils::audio::SampleConsumer;

use crate::{Emulator, Frame, KeyState, SCREEN_HEIGHT, SCREEN_WIDTH};

/// Size of the GBA BIOS in bytes.
const BIOS_SIZE: usize = 16 * 1024;

/// Bytes of the BIOS region.
///
/// The core indexes this buffer directly, so it must always be exactly
/// [`BIOS_SIZE`] long even when we boot by skipping the BIOS. A caller may
/// later supply a real dump through [`CoreEmulator::with_bios`].
fn default_bios() -> Box<[u8]> {
    vec![0u8; BIOS_SIZE].into_boxed_slice()
}

/// SHA-256 of the open-source replacement BIOS shipped in `bios/gba_bios.bin`.
///
/// This is the VBA-M / Normmatt replacement BIOS (GPL-2.0), not a Nintendo dump.
pub const REPLACEMENT_BIOS_SHA256: &str =
    "661a9afb93624f2c5e77d07dab137bd8d23cff79e8639c62fe621b49bb064749";

/// Candidate BIOS locations, in priority order: an explicit env override, then
/// the app-data folder, then the repo's gitignored `bios/` folder.
fn bios_candidates() -> Vec<std::path::PathBuf> {
    let mut candidates = Vec::new();
    if let Ok(p) = std::env::var("OEMU_BIOS") {
        candidates.push(std::path::PathBuf::from(p));
    }
    if let Ok(appdata) = std::env::var("APPDATA") {
        candidates.push(
            std::path::PathBuf::from(appdata)
                .join("open-emulingo")
                .join("gba_bios.bin"),
        );
    }
    // Walk up from the current exe and from the crate dir, so both `cargo run`
    // and an installed build find `bios/gba_bios.bin`.
    for base in [
        std::env::current_exe().ok().and_then(|p| p.parent().map(|d| d.to_path_buf())),
        std::env::current_dir().ok(),
    ]
    .into_iter()
    .flatten()
    {
        for ancestor in base.ancestors().take(6) {
            candidates.push(ancestor.join("bios").join("gba_bios.bin"));
        }
    }
    candidates
}

/// Loads the first valid 16 KiB BIOS that can be found.
///
/// Returns `None` when no BIOS is available, in which case the caller keeps the
/// zero-filled placeholder (boot will blank, but nothing panics).
pub fn find_bios() -> Option<Vec<u8>> {
    for path in bios_candidates() {
        let Ok(bytes) = std::fs::read(&path) else {
            continue;
        };
        if bytes.len() == BIOS_SIZE {
            return Some(bytes);
        }
    }
    None
}

/// A minimal, structurally valid 192-byte GBA header used as a blank cartridge.
/// It carries no game data; it only satisfies the loader's header validation
/// before a real ROM is supplied via [`CoreEmulator::load_rom`].
fn blank_cartridge_bytes() -> Box<[u8]> {
    let mut buf = vec![0u8; 0xC0];
    buf[0xA0..0xAC].copy_from_slice(b"BLANK       ");
    buf[0xAC..0xB0].copy_from_slice(b"BLNE");
    buf[0xB2] = 0x96;
    buf.into_boxed_slice()
}

/// Collection of samples drained from the core each frame.
struct AudioSink {
    consumer: SampleConsumer,
    pending: Vec<f32>,
}

impl AudioSink {
    fn drain(&mut self) {
        use rustboyadvance_utils::Consumer as _;
        // The consumer yields interleaved stereo i16 samples.
        while let Some(sample) = self.consumer.try_pop() {
            self.pending.push(sample as f32 / 32768.0);
        }
    }
}

/// Real GBA emulator, backed by `rustboyadvance-core`.
pub struct CoreEmulator {
    gba: GameBoyAdvance,
    frame: Frame,
    audio: AudioSink,
    last_battery: Option<Vec<u8>>,
    bios: Box<[u8]>,
}

impl CoreEmulator {
    /// Builds a stopped machine with no cartridge loaded.
    pub fn new() -> Self {
        let (audio_interface, consumer) = SimpleAudioInterface::create_channel(44_100, None);

        // A cart must exist to build the machine; a blank one is harmless and
        // is replaced by `load_rom` before any real frame is run.
        let cart = GamepakBuilder::new()
            .take_buffer(blank_cartridge_bytes())
            .without_backup_to_file()
            .build()
            .expect("blank cartridge should build");

        let bios = default_bios();
        let mut gba = GameBoyAdvance::new(bios.clone(), cart, audio_interface as DynAudioInterface);
        gba.skip_bios();

        Self {
            gba,
            frame: Frame::default(),
            audio: AudioSink {
                consumer,
                pending: Vec::new(),
            },
            last_battery: None,
            bios,
        }
    }

    /// Builds an emulator that uses the best BIOS found on disk.
    ///
    /// Falls back to the zero-filled placeholder when no BIOS is present.
    pub fn with_discovered_bios() -> Self {
        match find_bios() {
            Some(bios) => Self::new().with_bios(bios).unwrap_or_else(|_| Self::new()),
            None => Self::new(),
        }
    }

    /// Replaces the placeholder BIOS with a real dump (must be 16 KiB).
    ///
    /// The new BIOS takes effect on the next [`Emulator::load_rom`].
    pub fn with_bios(mut self, bios: Vec<u8>) -> Result<Self, String> {
        if bios.len() != BIOS_SIZE {
            return Err(format!(
                "BIOS must be exactly {BIOS_SIZE} bytes, got {}",
                bios.len()
            ));
        }
        self.bios = bios.into_boxed_slice();
        Ok(self)
    }

    /// The most recently rendered frame, without advancing the machine.
    pub fn frame(&self) -> &Frame {
        &self.frame
    }

    /// Copies the core's 0xRRGGBB frame buffer into our RGBA8 [`Frame`].
    fn blit(&mut self) {
        let src = self.gba.get_frame_buffer();
        let dst = &mut self.frame.0[..];
        for (i, px) in src.iter().take(SCREEN_WIDTH * SCREEN_HEIGHT).enumerate() {
            let r = ((px >> 16) & 0xFF) as u8;
            let g = ((px >> 8) & 0xFF) as u8;
            let b = (px & 0xFF) as u8;
            let o = i * 4;
            dst[o] = r;
            dst[o + 1] = g;
            dst[o + 2] = b;
            dst[o + 3] = 0xFF;
        }
    }
}

impl Default for CoreEmulator {
    fn default() -> Self {
        Self::new()
    }
}

impl Emulator for CoreEmulator {
    fn load_rom(&mut self, rom_data: &[u8]) -> Result<(), String> {
        let cart = GamepakBuilder::new()
            .buffer(rom_data)
            .without_backup_to_file()
            .build()
            .map_err(|e| format!("failed to build cartridge: {e}"))?;

        let (audio_interface, consumer) = SimpleAudioInterface::create_channel(44_100, None);
        let mut gba = GameBoyAdvance::new(
            self.bios.clone(),
            cart,
            audio_interface as DynAudioInterface,
        );
        gba.skip_bios();

        self.gba = gba;
        self.audio = AudioSink {
            consumer,
            pending: Vec::new(),
        };
        self.last_battery = None;
        Ok(())
    }

    fn run_frame(&mut self) -> &Frame {
        self.gba.frame();
        self.audio.drain();
        self.blit();
        &self.frame
    }

    fn set_key(&mut self, key: KeyState, pressed: bool) {
        let bit = 1u16 << key.index();
        let keys = self.gba.get_key_state_mut();
        // In the GBA KEYINPUT register a cleared bit means "pressed".
        if pressed {
            *keys &= !bit;
        } else {
            *keys |= bit;
        }
    }

    fn drain_audio(&mut self) -> Vec<f32> {
        std::mem::take(&mut self.audio.pending)
    }

    fn save_state(&self) -> Vec<u8> {
        self.gba.save_state().unwrap_or_default()
    }

    fn load_state(&mut self, state: &[u8]) -> Result<(), String> {
        self.gba
            .restore_state(state)
            .map_err(|e| format!("failed to restore state: {e}"))
    }

    fn battery_save(&self) -> Option<Vec<u8>> {
        self.last_battery.clone()
    }

    fn load_battery(&mut self, save_data: &[u8]) {
        self.last_battery = Some(save_data.to_vec());
    }

    fn sample_rate(&self) -> u32 {
        44_100
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The RGBA blit turns the core's 0xRRGGBB buffer into an opaque frame.
    ///
    /// A blank cartridge cannot execute real code, so this test drives
    /// [`CoreEmulator::blit`] directly instead of running the CPU.
    #[test]
    fn blit_produces_opaque_rgba_frame() {
        let mut emu = CoreEmulator::new();
        emu.blit();
        let bytes = emu.frame.as_bytes();
        assert_eq!(bytes.len(), 240 * 160 * 4);
        // Every pixel must be fully opaque after the blit.
        assert!(bytes.chunks_exact(4).all(|p| p[3] == 0xFF));
    }

    /// Pressing then releasing a key round-trips through the KEYINPUT register.
    #[test]
    fn key_press_and_release_roundtrip() {
        let mut emu = CoreEmulator::new();
        let released = *emu.gba.get_key_state();
        emu.set_key(KeyState::A, true);
        let pressed = *emu.gba.get_key_state();
        assert_eq!(pressed, released & !1);
        emu.set_key(KeyState::A, false);
        assert_eq!(*emu.gba.get_key_state(), released);
    }

    /// Boots a real ROM from `OEMU_ROM_EN` and checks that 120 frames render a
    /// non-uniform image (i.e. the game is actually drawing something).
    #[test]
    #[ignore = "needs OEMU_ROM_EN pointing at a real .gba file"]
    fn boots_real_rom_and_draws() {
        let path = std::env::var("OEMU_ROM_EN").expect("set OEMU_ROM_EN to an English .gba file");
        let rom = std::fs::read(&path).expect("read ROM");

        let mut emu = CoreEmulator::with_discovered_bios();
        emu.load_rom(&rom).expect("load ROM");

        for _ in 0..400 {
            emu.run_frame();
        }

        let frame = emu.frame();
        let bytes = frame.as_bytes();
        let first = &bytes[0..4];
        let uniform = bytes.chunks_exact(4).all(|p| p == first);
        assert!(
            !uniform,
            "400 frames of a real ROM should not be one solid colour"
        );
    }

    /// A BIOS of the wrong length is rejected rather than silently accepted.
    #[test]
    fn rejects_wrong_sized_bios() {
        let result = CoreEmulator::new().with_bios(vec![0u8; 1024]);
        assert!(result.is_err());
    }
}
