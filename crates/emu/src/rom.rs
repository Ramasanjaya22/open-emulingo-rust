use oemu_core_types::Lang;
use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RomError {
    #[error("ROM file too small (< 0xC0 bytes)")]
    TooSmall,
    #[error("Invalid GBA fixed header byte at 0xB2: expected 0x96, got 0x{0:02X}")]
    InvalidFixedByte(u8),
}

#[derive(Debug, Clone)]
pub struct RomInfo {
    pub sha256: [u8; 32],
    pub title: String,
    pub game_code: String,
    pub lang: Option<Lang>,
}

pub fn parse_rom(bytes: &[u8]) -> Result<RomInfo, RomError> {
    if bytes.len() < 0xC0 {
        return Err(RomError::TooSmall);
    }

    let fixed_byte = bytes[0xB2];
    if fixed_byte != 0x96 {
        return Err(RomError::InvalidFixedByte(fixed_byte));
    }

    // SHA-256 hash of entire ROM bytes
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let sha256: [u8; 32] = hasher.finalize().into();

    // Title at 0xA0..0xAC
    let title_bytes = &bytes[0xA0..0xAC];
    let title_str = String::from_utf8_lossy(title_bytes);
    let title = title_str
        .trim_matches(|c: char| c == '\0' || c.is_whitespace())
        .to_string();

    // Game code at 0xAC..0xB0
    let code_bytes = &bytes[0xAC..0xB0];
    let game_code = String::from_utf8_lossy(code_bytes).trim().to_string();

    // Lang from 4th char of game code (index 3) or overall code
    let lang = if game_code.len() >= 4 {
        Lang::from_game_code_char(game_code.as_bytes()[3])
    } else {
        None
    };

    Ok(RomInfo {
        sha256,
        title,
        game_code,
        lang,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rom_too_small() {
        let buf = vec![0u8; 0x50];
        assert!(matches!(parse_rom(&buf), Err(RomError::TooSmall)));
    }

    #[test]
    fn test_parse_rom_invalid_fixed_byte() {
        let mut buf = vec![0u8; 0xC0];
        buf[0xB2] = 0x00;
        assert!(matches!(
            parse_rom(&buf),
            Err(RomError::InvalidFixedByte(0x00))
        ));
    }

    #[test]
    fn test_parse_rom_valid_ja() {
        let mut buf = vec![0u8; 0xC0];
        buf[0xB2] = 0x96;
        // Title "POKEMON EMER" at 0xA0
        let title = b"POKEMON EMER";
        buf[0xA0..0xA0 + title.len()].copy_from_slice(title);
        // Game code "BPEJ" at 0xAC
        let code = b"BPEJ";
        buf[0xAC..0xAC + code.len()].copy_from_slice(code);

        let info = parse_rom(&buf).unwrap();
        assert_eq!(info.game_code, "BPEJ");
        assert_eq!(info.lang, Some(Lang::Ja));
        assert_eq!(info.title, "POKEMON EMER");
    }

    #[test]
    fn test_parse_rom_valid_en() {
        let mut buf = vec![0u8; 0xC0];
        buf[0xB2] = 0x96;
        let title = b"POKEMON EMER";
        buf[0xA0..0xA0 + title.len()].copy_from_slice(title);
        let code = b"BPEE";
        buf[0xAC..0xAC + code.len()].copy_from_slice(code);

        let info = parse_rom(&buf).unwrap();
        assert_eq!(info.game_code, "BPEE");
        assert_eq!(info.lang, Some(Lang::En));
    }
}
