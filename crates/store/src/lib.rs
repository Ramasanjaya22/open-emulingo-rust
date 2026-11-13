//! SQLite persistence for oemu (schema v1).
//!
//! The schema lives in `migrations/0001_init.sql` and is applied through
//! `PRAGMA user_version`, so opening an existing database is idempotent.

use std::path::Path;

use rusqlite::{Connection, Result};
use thiserror::Error;

pub mod settings;
pub use settings::{Settings, TargetLang};

/// Schema version this build expects. Bump together with a new migration file.
pub const SCHEMA_VERSION: i64 = 1;

/// The full schema for version 1.
const MIGRATION_0001: &str = include_str!("../migrations/0001_init.sql");

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("database schema is version {found}, but this build expects {expected}")]
    UnsupportedSchema { found: i64, expected: i64 },
}

/// A handle to the on-disk (or in-memory) database.
pub struct Store {
    pub conn: Connection,
}

impl std::fmt::Debug for Store {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Store").finish_non_exhaustive()
    }
}

impl Store {
    /// Opens (or creates) the database at `path` and applies migrations.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StoreError> {
        if let Some(parent) = path.as_ref().parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        Self::init(Connection::open(path)?)
    }

    /// Creates a throwaway database in memory, useful for tests.
    pub fn open_in_memory() -> Result<Self, StoreError> {
        Self::init(Connection::open_in_memory()?)
    }

    /// Shared setup: pragmas plus `user_version`-driven migrations.
    fn init(conn: Connection) -> Result<Self, StoreError> {
        conn.execute_batch("PRAGMA journal_mode = WAL; PRAGMA foreign_keys = ON;")?;

        let version: i64 = conn.query_row("PRAGMA user_version", [], |row| row.get(0))?;
        if version > SCHEMA_VERSION {
            return Err(StoreError::UnsupportedSchema {
                found: version,
                expected: SCHEMA_VERSION,
            });
        }

        if version < 1 {
            conn.execute_batch(MIGRATION_0001)?;
            conn.pragma_update(None, "user_version", SCHEMA_VERSION)?;
        }

        Ok(Self { conn })
    }

    /// Reads the schema version currently stored in the database.
    pub fn schema_version(&self) -> Result<i64, StoreError> {
        Ok(self
            .conn
            .query_row("PRAGMA user_version", [], |row| row.get(0))?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A fresh database reports the current schema version.
    #[test]
    fn fresh_db_has_current_schema_version() {
        let store = Store::open_in_memory().unwrap();
        assert_eq!(store.schema_version().unwrap(), SCHEMA_VERSION);
    }

    /// Every table named in the plan must exist after migration.
    #[test]
    fn all_tables_exist() {
        let store = Store::open_in_memory().unwrap();
        let expected = [
            "games",
            "regions",
            "translations",
            "lexicon",
            "cards",
            "reviews",
            "companions",
            "memories",
            "stats",
            "settings",
        ];
        for table in expected {
            let count: i64 = store
                .conn
                .query_row(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1",
                    [table],
                    |row| row.get(0),
                )
                .unwrap();
            assert_eq!(count, 1, "missing table {table}");
        }
    }

    /// Migrations are guarded by `user_version`, so re-opening is a no-op.
    #[test]
    fn reopening_is_idempotent() {
        let dir = std::env::temp_dir().join(format!("oemu-store-test-{}", std::process::id()));
        let path = dir.join("db.sqlite");
        let _ = std::fs::remove_dir_all(&dir);

        let first = Store::open(&path).unwrap();
        assert_eq!(first.schema_version().unwrap(), SCHEMA_VERSION);
        drop(first);

        let second = Store::open(&path).unwrap();
        assert_eq!(second.schema_version().unwrap(), SCHEMA_VERSION);

        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A database from a newer build must be refused, not silently migrated.
    #[test]
    fn newer_schema_is_rejected() {
        let conn = Connection::open_in_memory().unwrap();
        conn.pragma_update(None, "user_version", SCHEMA_VERSION + 1)
            .unwrap();

        match Store::init(conn) {
            Err(StoreError::UnsupportedSchema { .. }) => {}
            other => panic!("expected UnsupportedSchema, got {other:?}"),
        }
    }

    /// `translations` is unique per (game, region, target language).
    #[test]
    fn translations_are_unique_per_region_and_language() {
        let store = Store::open_in_memory().unwrap();
        let insert =
            "INSERT INTO translations (game_code, region_hash, target_lang, source_lang, json) \
                      VALUES ('BPEE', 7, 'id', 'en', '{}')";
        store.conn.execute(insert, []).unwrap();

        let duplicate = store.conn.execute(insert, []);
        assert!(
            duplicate.is_err(),
            "duplicate translation should be rejected"
        );
    }

    /// `lexicon.source` only accepts the three known provenance values.
    #[test]
    fn lexicon_source_is_constrained() {
        let store = Store::open_in_memory().unwrap();
        let bad = "INSERT INTO lexicon (lemma, lang, target_lang, gloss, source) \
                   VALUES ('a', 'ja', 'id', 'b', 'made-up')";
        assert!(store.conn.execute(bad, []).is_err());
    }
}
