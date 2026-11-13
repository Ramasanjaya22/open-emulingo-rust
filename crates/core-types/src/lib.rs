//! Tiny shared vocabulary types used across the oemu crates.
//!
//! These live in their own crate so that `emu`, `store` and later `screen` /
//! `translate` can all talk about a game's language without depending on each
//! other.

use serde::{Deserialize, Serialize};

/// The language a game (or a piece of on-screen text) is written in.
///
/// Japanese is the primary learning target; English is secondary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Lang {
    Ja,
    En,
}

impl Lang {
    /// Maps the fourth character of a GBA game code to a language.
    ///
    /// `BPEE` -> `En`, `BPEJ` -> `Ja`, anything else -> `None`.
    pub fn from_game_code_char(c: u8) -> Option<Self> {
        match c {
            b'J' => Some(Lang::Ja),
            b'E' => Some(Lang::En),
            _ => None,
        }
    }

    /// Short BCP-47-ish tag, used for model prompts and DB columns.
    pub fn code(self) -> &'static str {
        match self {
            Lang::Ja => "ja",
            Lang::En => "en",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Japanese game codes map to `Lang::Ja`.
    #[test]
    fn game_code_char_j_is_japanese() {
        assert_eq!(Lang::from_game_code_char(b'J'), Some(Lang::Ja));
    }

    /// English game codes map to `Lang::En`.
    #[test]
    fn game_code_char_e_is_english() {
        assert_eq!(Lang::from_game_code_char(b'E'), Some(Lang::En));
    }

    /// Unknown region letters (e.g. German `D`) are not a supported language.
    #[test]
    fn unknown_game_code_char_is_none() {
        assert_eq!(Lang::from_game_code_char(b'D'), None);
        assert_eq!(Lang::from_game_code_char(b'P'), None);
    }

    /// `code()` returns the tag stored in the database.
    #[test]
    fn code_tags() {
        assert_eq!(Lang::Ja.code(), "ja");
        assert_eq!(Lang::En.code(), "en");
    }
}
