//! Integration tests for GBA ROM header parsing (T1.1).
//!
//! The synthetic ROMs below are hand-built 0xC0-byte buffers. No real ROM bytes
//! are ever committed. The two `#[ignore]`d tests read real ROMs from the
//! environment so they can be run locally:
//!
//! ```text
//! OEMU_ROM_JA=... OEMU_ROM_EN=... cargo test -p oemu-emu -- --ignored
//! ```

use oemu_core_types::Lang;
use oemu_emu::rom::{parse_rom, RomError};

/// Builds a minimal but valid 192-byte GBA header with the given game code.
fn synthetic_rom(game_code: &[u8; 4]) -> Vec<u8> {
    let mut buf = vec![0u8; 0xC0];
    buf[0xB2] = 0x96; // fixed value
    buf[0xA0..0xAC].copy_from_slice(b"POKEMON EMER");
    buf[0xAC..0xB0].copy_from_slice(game_code);
    buf
}

/// `BPEE` is the English Emerald release.
#[test]
fn parses_english_emerald_code() {
    let info = parse_rom(&synthetic_rom(b"BPEE")).expect("valid header");
    assert_eq!(info.game_code, "BPEE");
    assert_eq!(info.lang, Some(Lang::En));
    assert_eq!(info.title, "POKEMON EMER");
}

/// `BPEJ` is the Japanese Emerald release.
#[test]
fn parses_japanese_emerald_code() {
    let info = parse_rom(&synthetic_rom(b"BPEJ")).expect("valid header");
    assert_eq!(info.game_code, "BPEJ");
    assert_eq!(info.lang, Some(Lang::Ja));
}

/// A German code parses but has no supported language.
#[test]
fn german_code_has_no_language() {
    let info = parse_rom(&synthetic_rom(b"BPED")).expect("valid header");
    assert_eq!(info.lang, None);
}

/// The fixed byte at 0xB2 must be 0x96.
#[test]
fn rejects_wrong_fixed_byte() {
    let mut rom = synthetic_rom(b"BPEE");
    rom[0xB2] = 0x00;
    match parse_rom(&rom) {
        Err(RomError::InvalidFixedByte(0x00)) => {}
        other => panic!("expected InvalidFixedByte(0x00), got {other:?}"),
    }
}

/// Anything shorter than the header is rejected.
#[test]
fn rejects_truncated_rom() {
    let rom = vec![0u8; 0xBF];
    assert!(matches!(parse_rom(&rom), Err(RomError::TooSmall)));
}

/// The SHA-256 is of the whole ROM and differs between two ROMs.
#[test]
fn sha256_is_stable_and_distinct() {
    let en = parse_rom(&synthetic_rom(b"BPEE")).unwrap();
    let ja = parse_rom(&synthetic_rom(b"BPEJ")).unwrap();
    let en_again = parse_rom(&synthetic_rom(b"BPEE")).unwrap();

    assert_eq!(en.sha256, en_again.sha256);
    assert_ne!(en.sha256, ja.sha256);
}

/// Reads the real English ROM from `OEMU_ROM_EN` and checks its game code.
#[test]
#[ignore = "needs OEMU_ROM_EN pointing at a real .gba file"]
fn real_english_rom_has_code_bpee() {
    let path = std::env::var("OEMU_ROM_EN").expect("set OEMU_ROM_EN to an English .gba file");
    let bytes = std::fs::read(&path).expect("read ROM");
    let info = parse_rom(&bytes).expect("valid ROM header");
    assert_eq!(info.game_code, "BPEE");
    assert_eq!(info.lang, Some(Lang::En));
}

/// Reads the real Japanese ROM from `OEMU_ROM_JA` and checks its game code.
#[test]
#[ignore = "needs OEMU_ROM_JA pointing at a real .gba file"]
fn real_japanese_rom_has_code_bpej() {
    let path = std::env::var("OEMU_ROM_JA").expect("set OEMU_ROM_JA to a Japanese .gba file");
    let bytes = std::fs::read(&path).expect("read ROM");
    let info = parse_rom(&bytes).expect("valid ROM header");
    assert_eq!(info.game_code, "BPEJ");
    assert_eq!(info.lang, Some(Lang::Ja));
}
