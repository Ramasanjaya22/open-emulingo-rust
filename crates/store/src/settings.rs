use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TargetLang {
    Id,
    En,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    pub target_lang: TargetLang,
    pub auto_translate: bool,
    pub llm_url: String,
    pub bios_path: Option<PathBuf>,
    pub enable_2b: bool,
    pub enable_9b: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            target_lang: TargetLang::Id,
            auto_translate: true,
            llm_url: "http://127.0.0.1:8080".to_string(),
            bios_path: None,
            enable_2b: false,
            enable_9b: false,
        }
    }
}

impl Settings {
    pub fn save(&self, conn: &Connection) -> Result<()> {
        let json = serde_json::to_string(self).unwrap();
        conn.execute(
            "INSERT INTO settings (key, value) VALUES ('app_settings', ?1) ON CONFLICT(key) DO UPDATE SET value = ?1",
            [json],
        )?;
        Ok(())
    }

    pub fn load(conn: &Connection) -> Result<Self> {
        let res: Result<String> = conn.query_row(
            "SELECT value FROM settings WHERE key = 'app_settings'",
            [],
            |row| row.get(0),
        );
        match res {
            Ok(json) => {
                let settings = serde_json::from_str(&json).unwrap_or_default();
                Ok(settings)
            }
            Err(_) => Ok(Self::default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Store;

    #[test]
    fn test_settings_roundtrip() {
        let store = Store::open_in_memory().unwrap();
        let settings = Settings {
            auto_translate: false,
            llm_url: "http://127.0.0.1:9090".to_string(),
            ..Settings::default()
        };

        settings.save(&store.conn).unwrap();
        let loaded = Settings::load(&store.conn).unwrap();

        assert!(!loaded.auto_translate);
        assert_eq!(loaded.llm_url, "http://127.0.0.1:9090");
        assert_eq!(loaded.target_lang, TargetLang::Id);
    }

    /// Defaults match the plan: Indonesian target, auto-translate on, local LLM.
    #[test]
    fn defaults_match_plan() {
        let defaults = Settings::default();
        assert_eq!(defaults.target_lang, TargetLang::Id);
        assert!(defaults.auto_translate);
        assert_eq!(defaults.llm_url, "http://127.0.0.1:8080");
        assert_eq!(defaults.bios_path, None);
        assert!(!defaults.enable_2b);
        assert!(!defaults.enable_9b);
    }

    /// An empty settings table yields the defaults rather than an error.
    #[test]
    fn missing_settings_load_defaults() {
        let store = Store::open_in_memory().unwrap();
        let loaded = Settings::load(&store.conn).unwrap();
        assert_eq!(loaded.target_lang, TargetLang::Id);
        assert_eq!(loaded.llm_url, "http://127.0.0.1:8080");
    }
}
