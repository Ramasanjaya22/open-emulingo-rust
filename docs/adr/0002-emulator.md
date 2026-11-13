# ADR 0002 — GBA emulator core

- **Status:** accepted
- **Date:** 2026-11-13
- **Task:** T1.2
- **Supersedes:** the T0.6 spike outcome described in `docs/plan/phase-0-spike.md`

## Context

The product needs to run GBA games offline, on Windows, at the full 59.73 Hz
frame rate, while the same process also captures the screen for OCR. The
emulator must sit behind a stable trait so the core can be swapped later without
touching the UI or the translation pipeline.

Constraints from `AGENTS.md`:

- Rust only (no Python, no React) and no network access at runtime.
- Prefer a small, mature crate over a bespoke core.
- Clean-room: no code, text or assets copied from emulingo.com.

## Options considered

1. **`rustboyadvance-ng`** (`michelhe/rustboyadvance-ng`) — pure Rust, MIT,
   ~670 stars, actively maintained. Ships `rustboyadvance-core`, an `Arm7tdmi`
   CPU, PPU, DMA, timers, sound and cartridge backup (SRAM / Flash 64K / Flash
   128K / EEPROM + RTC).
2. **mGBA via FFI** — highly accurate, MPL-2.0, but pulls in a large C build and
   a C toolchain, and complicates shipping on Windows.
3. **Write our own core** — out of scope for T1.x.

## Decision

Use **`rustboyadvance-ng`** through the `oemu-emu` crate's `Emulator` trait, via
`crates/emu/src/adapter_rustboyadvance.rs`.

The core is **not published on crates.io**, so it is consumed as a pinned git
dependency (commit `1d7ff23a`) in `crates/emu/Cargo.toml`. Pinning matters:
the crate is pre-1.0 and has no stability guarantee.

Only `adapter_rustboyadvance.rs` may name core types. Everything else uses
`core::Frame`, `core::KeyState` and the `Emulator` trait, so a future swap to
mGBA-FFI touches exactly one file.

## BIOS

The core indexes a BIOS buffer directly and will not boot a commercial game
without one. Shipping a Nintendo dump is not legally distributable, so
`scripts/setup-bios.ps1` downloads the **open-source replacement BIOS**
(VBA-M / Normmatt, GPL-2.0) that the core's own README recommends, pins its
SHA-256, and writes it to the gitignored `bios/gba_bios.bin`.

`CoreEmulator::with_discovered_bios()` searches, in order:

1. `$OEMU_BIOS`
2. `%APPDATA%/open-emulingo/gba_bios.bin`
3. `bios/gba_bios.bin` walking up from the executable and the working directory

With no BIOS the emulator still constructs and runs; the screen stays blank
(the GPU's force-blank colour) rather than panicking. The UI shows a
"BIOS tidak ditemukan" badge so this is never a silent failure.

## Consequences

- The whole core is single threaded (`Rc` everywhere), so the emulator has to
  own a dedicated thread. `app/src-tauri/src/emulator_thread.rs` does this and
  communicates over `mpsc`; the core is never moved or shared.
- Upstream is pre-1.0. The pinned commit must be re-validated before bumping,
  using `cargo test --workspace -- --ignored` with `OEMU_ROM_EN` / `OEMU_ROM_JA`.
- Verified on this machine: both `BPEE` (English) and `BPEJ` (Japanese) Emerald
  boot through the intro and render distinct frames.

## Verification

```
cargo test -p oemu-emu -- --ignored          # ROM header + real boot
cargo test -p oemu-app -- --ignored          # full host pipeline
```

Both read ROM paths from `OEMU_ROM_EN` / `OEMU_ROM_JA`; no ROM bytes are
committed.
