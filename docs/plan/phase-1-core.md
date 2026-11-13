# Phase 1 — Core: workspace, emulator, storage, app shell

Precondition: `docs/adr/0001-pipeline.md` and `0002-emulator.md` accepted.
Interfaces: `00-master-plan.md` §3 "Interface inti".

---

### T1.1 — Workspace + `emu` crate skeleton + ROM header
- **Worker:** free_pi
- **Depends:** T0.7
- **Files:** `Cargo.toml` (members `crates/*`), `crates/emu/{Cargo.toml,src/lib.rs,src/rom.rs}`, `crates/emu/tests/rom_header.rs`.
- **Spec:**
  - `pub struct RomInfo { pub sha256: [u8; 32], pub title: String, pub game_code: String, pub lang: Option<Lang> }` and `pub fn parse_rom(bytes: &[u8]) -> Result<RomInfo, RomError>`.
  - Header: title = bytes `0xA0..0xAC` (ASCII, trim NUL/space), game code = `0xAC..0xB0`. Lang from 4th char of game code: `J`→Ja, `E`→En, else `None`. Validate fixed byte `0xB2 == 0x96`; `RomError::TooSmall` if < 0xC0 bytes.
  - `pub enum Lang { Ja, En }` lives in a new tiny crate `crates/core-types` (also used by later crates).
  - Deps: `sha2`, `thiserror`.
- **Tests:** synthetic 0xC0-byte buffers (no ROM bytes!): valid JA/EN codes, `D` code → `None`, bad fixed byte, too small. `#[ignore]` test reading `OEMU_ROM_EN` → `game_code == "BPEE"`, and `OEMU_ROM_JA` → `"BPEJ"`.
- **Done when:** AGENTS.md commands green; `OEMU_ROM_EN=... OEMU_ROM_JA=... cargo test -p emu -- --ignored` passes.

### T1.2 — `Emulator` trait + adapter
- **Worker:** clinepass
- **Depends:** T1.1
- **Files:** `crates/emu/src/{lib.rs,core.rs,adapter_<core>.rs}`, `crates/emu/tests/boot.rs`.
- **Spec:** trait from master plan §3 (`load_rom`, `run_frame -> &Frame`, `set_keys`, `drain_audio`, `save_state`, `load_state`) + `fn battery_save(&self) -> Option<Vec<u8>>`, `fn load_battery(&mut self, &[u8])`. `Frame` = `[u8; 240*160*4]` RGBA8 newtype. `KeyState` bitflags for 10 GBA keys. Adapter wraps the core chosen in ADR 0002; core types never leak out of `adapter_*.rs`. BIOS path from settings, default bundled-replacement path.
- **Tests (ignored, need ROM):** boot 600 frames → frame not all-one-color; save_state → run 60 → load_state → next frame identical to the one recorded after save.
- **Done when:** green + ignored tests pass with `OEMU_ROM_JA` (and `OEMU_ROM_EN`).

### T1.3 — `store` crate (SQLite schema v1)
- **Worker:** free_pi
- **Depends:** T1.1
- **Files:** `crates/store/{Cargo.toml,src/lib.rs,src/migrations/0001_init.sql}`, `crates/store/tests/schema.rs`.
- **Spec:** `rusqlite` (feature `bundled`), WAL mode. `Store::open(path)`, `Store::open_in_memory()`, migrations via `PRAGMA user_version`. Tables:
  - `games(sha256 BLOB PK, title, game_code, lang, added_at)`
  - `regions(game_code, name, x, y, w, h, PRIMARY KEY(game_code,name))`
  - `translations(id INTEGER PK, game_code, region_hash INTEGER, target_lang, source_lang, json TEXT, created_at, UNIQUE(game_code,region_hash,target_lang))`
  - `lexicon(lemma, lang, target_lang, gloss, source TEXT CHECK(source IN('jmdict','kaikki','llm')), PRIMARY KEY(lemma,lang,target_lang))`
  - `cards(id PK, lemma, lang, example, image BLOB, fsrs_state TEXT, due_at)`, `reviews(card_id, rating, reviewed_at)`
  - `companions(id PK, name, persona, avatar BLOB, lang)`, `memories(companion_id, fact, created_at)`
  - `stats(day TEXT PK, xp, reviews, translations)`, `settings(key PK, value)`
- **Tests:** open twice = idempotent; `user_version == 1`; unique constraint on translations; settings round-trip.
- **Done when:** green.

### T1.4 — Tauri 2 shell + SvelteKit UI: play a ROM
- **Worker:** clinepass
- **Depends:** T1.2, T1.3
- **Files:** `app/src-tauri/**`, `app/ui/**` (SvelteKit + TS, `adapter-static`, pnpm).
- **Spec:**
  - Emulator runs on its own thread at 59.73 Hz (GBA rate) with a frame pacer; frames pushed to UI through `tauri::ipc::Channel` as raw RGBA bytes; UI draws to a `<canvas>` (240×160, CSS scaled integer, `image-rendering: pixelated`) in `requestAnimationFrame`.
  - Audio: `cpal` output, resample core audio to device rate, ring buffer.
  - Input: keyboard (arrows, Z=A, X=B, Enter=Start, Backspace=Select, A/S=L/R), configurable later; gamepad via `gilrs`.
  - Open ROM via file dialog → `parse_rom` → `games` row; battery save written to `%APPDATA%/open-emulingo/saves/<sha256>.sav` on exit and every 30 s.
  - Save/load state hotkeys F5/F9. FPS overlay toggle F3.
  - UI style: light mode, system font, generous spacing, no gradients (clean Apple-like).
- **Done when:** `pnpm -C app/ui build` + `cargo tauri build --debug` succeed; **HUMAN** check: play Emerald 5 min, FPS overlay steady ~60, sound OK, in-game save survives app restart.

### T1.5 — Settings model
- **Worker:** free_pi
- **Depends:** T1.3
- **Files:** `crates/store/src/settings.rs`, tests.
- **Spec:** typed `Settings { target_lang: TargetLang /* Id | En, default Id */, auto_translate: bool /* default true */, llm_url: String /* default http://127.0.0.1:8080 */, bios_path: Option<PathBuf>, enable_2b: bool, enable_9b: bool /* both false */ }`, load/save via `settings` table as JSON per key; unknown keys ignored.
- **Done when:** green; default round-trip test.
