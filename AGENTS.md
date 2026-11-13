# AGENTS.md — open-emulingo-rust

Rules for every coding agent (free-pi, ClinePass/Cline, Hermes). Read fully before any change.

## Project
Offline clean-room clone of emulingo.com: play GBA games, translate the screen, learn vocab.
Learn languages: **Japanese (primary, all eval gates)**, English (secondary). German dropped. Translate into: Indonesian or English (user setting).
Windows 11 first. 100% offline at runtime. Plan: `docs/plan/` (start at `docs/plan/README.md`).

## Hard rules
1. **Never commit ROMs, BIOS, saves, or models.** `*.gba`, `*.sav`, `*.ss*`, `models/`, `Pokemon*/` stay gitignored. Never copy ROM bytes into `tests/fixtures`.
2. **No Python, no React** anywhere (scripts, tooling, UI). Scripts = PowerShell (`.ps1`) or Rust `xtask`/examples.
3. **No network at runtime.** Only `127.0.0.1` (llama-server). Downloads happen only in `scripts/setup-models.ps1`.
4. Clean-room: never copy code, text, assets, or branding from emulingo.com. Product name must not contain "Emulingo".
5. Do only the task packet you were given (`docs/plan/phase-*.md`, task id `Tn.m`). No drive-by refactors, no extra features.
6. Add a dependency only if the task packet names it, or explain why in the PR/summary. Prefer std → small mature crate.
7. Don't commit or push unless the task packet says so. Never rewrite history.

## Stack
- Rust stable, Cargo workspace (`crates/*`, `app/src-tauri`).
- UI: Tauri 2 + SvelteKit + TypeScript + pnpm (`adapter-static`), light mode, clean minimal design.
- Inference: `llama-server` (llama.cpp Vulkan build) sidecar, OpenAI-compatible API, model Qwen3.5-4B Q4_K_M + mmproj Q8_0.
- Voice: whisper-rs (Whisper small), sherpa-rs (Kokoro en/ja), cpal.
- Data: rusqlite (bundled), fsrs, lindera.

## Commands (must be green before you report done)
```
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo test --workspace -- --list   # confirm your new tests are registered
```
UI (when touched): `pnpm -C app/ui check && pnpm -C app/ui build`.

## Test discipline
- TDD: write the failing test first, then the code.
- `#[test]` directly above `fn`, exactly once. Doc comment above `#[test]`.
- Tests needing a real ROM or model are `#[ignore]` and read paths from env:
  `OEMU_ROM_JA`, `OEMU_ROM_EN`, `OEMU_LLM_URL` (default `http://127.0.0.1:8080`).

## Local assets (gitignored, already on disk)
- EN ROM `BPEE`: `Pokemon - Emerald Version (USA, Europe)/Pokemon - Emerald Version (USA, Europe).gba` (SHA-256 `a9dec84dfe7f62ab2220bafaef7479da0929d066ece16a6885f6226db19085af`)
- JA ROM `BPEJ`: `Pokemon Emerald (J)(Independent)/1649 - Pokemon Emerald (J)(Independent).gba` (SHA-256 `33f5610b9186b4add09fef68895deb00f552b997b3d133b5a961e5123506343c`)

## Reporting (end of every task)
Max 10 lines: task id, files changed, commands run + result (pass/fail counts), anything not done, open questions. No full diffs in chat.

## Shell
Windows host. Paths contain spaces and parentheses — always quote them. Use forward slashes for native tools.
