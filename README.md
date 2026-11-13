# OpenKotoba

Play a GBA game and learn Japanese from what is on screen. This file is the
orientation map; the detailed task packets live in `docs/plan/`.

## What it is

A 100% offline desktop app (Tauri 2 + SvelteKit frontend, Rust backend) that

1. runs a GBA game in a pure-Rust emulator,
2. reads the on-screen text,
3. translates it into Indonesian or English with a local LLM,
4. turns words into study cards.

Primary learning target is Japanese; English is secondary. German is dropped.

## Layout

| Path | What lives there |
|---|---|
| `crates/core-types` | `Lang`, shared by every other crate |
| `crates/emu` | GBA core behind the `Emulator` trait, ROM header parsing |
| `crates/store` | SQLite schema + typed settings |
| `crates/llm` | Client for the local `llama-server` |
| `crates/screen` | Screen capture, OCR (later phases) |
| `crates/translate` | Translation pipeline (later phases) |
| `app/src-tauri` | Desktop shell, emulator thread, IPC commands |
| `app/ui` | SvelteKit UI, canvas screen, keyboard input |
| `docs/plan` | Phase plans and task packets |

## Getting it running

```bash
# 1. One-time assets (gitignored, never committed)
powershell -ExecutionPolicy Bypass -File scripts/setup-bios.ps1

# 2. Rust checks
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

# 3. Frontend
pnpm -C app/ui install      # first time only
pnpm -C app/ui check
pnpm -C app/ui build

# 4. Launch the desktop app
cargo tauri dev
```

ROMs are never committed. Point the app at your own legally obtained `.gba`
files with **Open ROM…** in the toolbar.

## Controls

| Key | Action |
|---|---|
| Arrow keys | D-pad |
| `Z` | A |
| `X` | B |
| `Enter` | Start |
| `Backspace` | Select |
| `A` / `S` | L / R |
| `F5` / `F9` | Save / load state |
| `F3` | Toggle the FPS overlay |

## Emulator core

The GBA core is [`rustboyadvance-ng`] (MIT), pinned by commit in
`crates/emu/Cargo.toml`. It is not on crates.io, so it is consumed as a git
dependency. See `docs/adr/0002-emulator.md`.

A BIOS is required for real games to boot. `scripts/setup-bios.ps1` downloads
the **open-source replacement BIOS** (VBA-M / Normmatt, GPL-2.0) that the core
itself recommends. It is not a Nintendo dump and it is gitignored.

[`rustboyadvance-ng`]: https://github.com/michelhe/rustboyadvance-ng

## Hard rules

- No ROMs, BIOS, saves, or models in git.
- No Python and no React anywhere.
- No network at runtime except `127.0.0.1`.
- Clean-room: nothing copied from emulingo.com, and the product name must never
  contain "Emulingo".

Full rules: `AGENTS.md`.
