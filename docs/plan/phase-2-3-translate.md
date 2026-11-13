# Phase 2 + 3 — Manual translate, then auto-translate

Precondition: Phase 1 merged, `scripts/run-llm.ps1` serving Qwen3.5-4B on `127.0.0.1:8080`.

---

## Phase 2 — Manual translate

### T2.1 — `screen` crate: crop, upscale, PNG
- **Worker:** free_pi
- **Depends:** T1.2
- **Files:** `crates/screen/{Cargo.toml,src/lib.rs}`, tests.
- **Spec:** `pub struct Rect { x, y, w, h }`; `crop(frame:&Frame, r:Rect) -> RgbaImage`; `upscale_nearest(img, factor:u32)`; `to_png(img) -> Vec<u8>`. Dep: `image` (png feature only). Reject rects outside 240×160.
- **Tests:** full-frame 4× → 960×640; crop bounds error; nearest preserves exact pixel colors (no new colors).
- **Done when:** green.

### T2.2 — `llm` crate: client for llama-server
- **Worker:** clinepass
- **Depends:** T1.5
- **Files:** `crates/llm/**`, tests with `wiremock`.
- **Spec:**
  - `LlmClient::new(base_url)`; reject non-loopback hosts (`127.0.0.1`, `::1`, `localhost` only) → `LlmError::NonLocalHost`.
  - `async fn vision_json<T: DeserializeOwned>(&self, tier: Tier, system:&str, user:&str, png:&[u8], schema:&serde_json::Value) -> Result<T>` → POST `/v1/chat/completions`, image as `data:image/png;base64,...`, `response_format: {type:"json_schema", json_schema:{schema}}`, `temperature: 0`, `chat_template_kwargs: {enable_thinking:false}`.
  - `async fn chat_stream(&self, tier, messages) -> impl Stream<Item=Result<String>>` (SSE `data:` lines, `[DONE]`).
  - `Tier` → model/URL map from settings; default every tier → the single 4B server.
  - Timeout 20 s; cancellation by dropping the future.
  - Deps: `reqwest` (rustls, json, stream), `tokio`, `serde`, `base64`, `futures`.
- **Tests (wiremock):** request body has json_schema + image data URL; SSE parse incl. split chunks; non-local URL rejected; HTTP 500 → error, not panic. One `#[ignore]` live test against `OEMU_LLM_URL`.
- **Done when:** green.

### T2.3 — `translate` crate: prompt + parse
- **Worker:** clinepass
- **Depends:** T2.1, T2.2
- **Files:** `crates/translate/**`, `crates/translate/prompts/{ocr_translate.txt}`, `crates/translate/examples/eval.rs`.
- **Spec:**
  - Types `ScreenTranslation / Line / Token` (master plan §3) with serde + JSON Schema (hand-written `schema()` fn, no schema-derive dep).
  - One call does OCR + translation: system prompt states source game language (from `RomInfo.lang`), target language (`id`/`en`), asks for `reading` (hiragana for ja, else null) and tokens with lemma + gloss in the target language. Source=en & target=en → "simplify + explain" instruction.
  - Validation: empty `lines` allowed (no text on screen); malformed JSON → 1 retry with "Return valid JSON only." then `TranslateError::BadOutput`.
  - `examples/eval.rs` reruns T0.3 metrics through this crate (must match T0.3 numbers ± noise).
- **Tests:** parse golden JSON fixtures (hand-written, ja + en); prompt contains target lang; en→en uses explain mode.
- **Done when:** green; `cargo run -p translate --example eval -- --lang ja` prints CER ≤ T0.3 gate.

### T2.4 — UI: translate overlay + history
- **Worker:** clinepass
- **Depends:** T2.3, T1.4
- **Files:** `app/src-tauri/src/commands/translate.rs`, `app/ui/src/lib/components/{TranslatePanel,History}.svelte`.
- **Spec:** hotkey `T` / button: freeze frame → translate whole screen → side panel lists lines: original, reading (furigana style for ja), translation; tokens clickable (popup with lemma/gloss, "Add to deck" stub until Phase 4). Target-lang toggle `ID | EN` in header (writes setting). History panel from `translations` table. Loading state + error message (e.g. "LLM server not running — start scripts/run-llm.ps1").
- **Done when:** builds; **HUMAN**: Emerald EN + JA intro dialog translated to ID and EN.

---

## Phase 3 — Auto-translate

### T3.1 — Text-change detector
- **Worker:** free_pi
- **Depends:** T2.1
- **Files:** `crates/screen/src/detect.rs`, tests.
- **Spec:**
  - `dhash64(img) -> u64` (9×8 grayscale difference hash) and `region_hash(frame, rect) -> u64` (exact: FxHash of raw region bytes; used as cache key).
  - `StableTextDetector { rect, stable_frames: u32 /*12*/, blank_threshold }` with `fn push(&mut self, frame:&Frame) -> Option<Trigger{ region_hash }>`: fires **once** when the region changed and then stayed identical for `stable_frames` frames; never fires for a blank/uniform box; resets when region changes again.
- **Tests:** synthetic frame sequences: typewriter (text grows 1 glyph/2 frames) → exactly 1 trigger after final glyph + 12 frames; same text held 600 frames → still 1; blank box → 0; two dialogs back-to-back → 2.
- **Done when:** green.

### T3.2 — Emerald region presets
- **Worker:** free_pi
- **Depends:** T3.1, T1.3
- **Files:** `crates/screen/src/presets.rs`, tests.
- **Spec:** `fn presets(game_code:&str) -> &'static [(&'static str, Rect)]`: `BPEJ` (measure first) and `BPEE` → `dialog` (bottom text box, measure from `eval/screens` — default guess `Rect{x:0,y:112,w:240,h:48}`), `battle` (bottom battle message box). Unknown game → `dialog` default only. User overrides stored in `regions` table win over presets.
- **Done when:** green; preset rects verified by **HUMAN** against 5 screenshots each (overlay drawn by a tiny `examples/draw_rect.rs`).

### T3.3 — Auto-translate pipeline + cache
- **Worker:** clinepass
- **Depends:** T3.1, T3.2, T2.3
- **Files:** `crates/translate/src/auto.rs`, `app/src-tauri/src/auto.rs`.
- **Spec:** emulator thread sends frames to detector (no copy if possible: send only the region bytes); on trigger: cache lookup `(game_code, region_hash, target_lang)` → hit shows instantly; miss → crop + 4× → `translate` at `Tier::Fast2B` (maps to 4B unless enable_2b); max 1 in-flight request, a newer trigger cancels the older; result stored in `translations`. Must never block or slow the emulator thread (bounded channel, drop frames to detector if busy).
- **Tests:** pipeline with fake LLM (trait object): cache hit makes 0 LLM calls; cancel-on-new-trigger; emulator-side send never blocks (bounded try_send).
- **Done when:** green; **HUMAN** 10 min Emerald EN + JA: overlay appears for each new dialog, no duplicates, FPS stays ~60; p50 trigger→overlay ≤ 1.8 s logged.

### T3.4 — Region editor UI
- **Worker:** clinepass
- **Depends:** T3.3
- **Spec:** drag a rectangle on the paused frame, name it, save to `regions` for the current game; toggle which regions auto-translate.
- **Done when:** builds; **HUMAN** creates a custom region and auto-translate uses it.
