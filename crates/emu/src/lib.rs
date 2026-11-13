//! GBA emulation surface for oemu.
//!
//! Everything outside this crate talks to the [`Emulator`] trait only. The
//! concrete core lives in [`adapter_rustboyadvance`] and never leaks its types.

pub mod adapter_rustboyadvance;
pub mod rom;

pub use oemu_core_types::Lang;
pub use rom::{parse_rom, RomError, RomInfo};

/// One of the ten physical GBA buttons.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyState {
    A = 0,
    B = 1,
    Select = 2,
    Start = 3,
    Right = 4,
    Left = 5,
    Up = 6,
    Down = 7,
    R = 8,
    L = 9,
}

impl KeyState {
    /// All ten keys, in index order.
    pub const ALL: [KeyState; 10] = [
        KeyState::A,
        KeyState::B,
        KeyState::Select,
        KeyState::Start,
        KeyState::Right,
        KeyState::Left,
        KeyState::Up,
        KeyState::Down,
        KeyState::R,
        KeyState::L,
    ];

    /// Index into [`KeyState::ALL`], used to address a key bit.
    pub fn index(self) -> usize {
        self as usize
    }
}

/// Screen width of a GBA in pixels.
pub const SCREEN_WIDTH: usize = 240;
/// Screen height of a GBA in pixels.
pub const SCREEN_HEIGHT: usize = 160;

/// A single rendered frame in RGBA8, row-major, 240x160.
#[derive(Clone)]
pub struct Frame(pub Box<[u8; SCREEN_WIDTH * SCREEN_HEIGHT * 4]>);

impl Default for Frame {
    fn default() -> Self {
        Self(Box::new([0; SCREEN_WIDTH * SCREEN_HEIGHT * 4]))
    }
}

impl Frame {
    /// Length of the underlying buffer in bytes.
    pub const LEN: usize = SCREEN_WIDTH * SCREEN_HEIGHT * 4;

    /// The raw RGBA8 bytes, ready to be shipped to the UI.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0[..]
    }
}

/// The one interface the rest of the app uses to drive a GBA.
pub trait Emulator {
    /// Loads ROM bytes and resets the machine.
    fn load_rom(&mut self, rom_data: &[u8]) -> Result<(), String>;
    /// Advances one frame (1/59.73 s) and returns the freshly rendered image.
    fn run_frame(&mut self) -> &Frame;
    /// Presses or releases one button.
    fn set_key(&mut self, key: KeyState, pressed: bool);
    /// Returns all stereo samples produced since the last call.
    fn drain_audio(&mut self) -> Vec<f32>;
    /// Serialises the entire machine state.
    fn save_state(&self) -> Vec<u8>;
    /// Restores a state produced by [`Emulator::save_state`].
    fn load_state(&mut self, state: &[u8]) -> Result<(), String>;
    /// The game's battery-backed save data (`.sav`), if the cartridge has any.
    fn battery_save(&self) -> Option<Vec<u8>>;
    /// Writes previously saved battery data back into the cartridge.
    fn load_battery(&mut self, save_data: &[u8]);
    /// Run all remaining audio out of the ring so the buffer does not grow.
    fn sample_rate(&self) -> u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A frame is exactly 240x160 RGBA8 bytes.
    #[test]
    fn frame_is_rgba8_sized() {
        let frame = Frame::default();
        assert_eq!(frame.as_bytes().len(), 240 * 160 * 4);
        assert_eq!(Frame::LEN, 240 * 160 * 4);
    }

    /// Key indices are stable and cover all ten buttons without duplicates.
    #[test]
    fn key_indices_are_stable() {
        let mut seen = [false; 10];
        for key in KeyState::ALL {
            let idx = key.index();
            assert!(idx < 10);
            assert!(!seen[idx], "duplicate index for {key:?}");
            seen[idx] = true;
        }
        assert!(seen.iter().all(|s| *s));
        assert_eq!(KeyState::A.index(), 0);
        assert_eq!(KeyState::L.index(), 9);
    }
}
