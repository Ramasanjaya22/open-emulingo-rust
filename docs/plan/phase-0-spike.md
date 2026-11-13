# Phase 0 — Spike (go/no-go)

Goal: prove with numbers that one Qwen3.5-4B + small voice models + a Rust GBA core are enough, before any product code.
Output: `eval/` data + reports + `docs/adr/0001-pipeline.md`. Code here is throwaway-quality but lives in the repo (`spikes/`), Rust or PowerShell only.

Packet format: Worker · Depends · Files · Spec · Done when · Out of scope.

---

### T0.0 — Repo bootstrap
- **Worker:** free_pi
- **Depends:** —
- **Files:** create `.gitignore`, `README.md`, `Cargo.toml` (empty workspace `members = []`), `rust-toolchain.toml` (`channel = "stable"`).
- **Spec:**
  - `.gitignore` FIRST, before `git init`: `*.gba`, `*.sav`, `*.ss[0-9]`, `/Pokemon*/`, `/models/`, `/target/`, `/eval/screens/*.png` (screens contain game art → local only), `node_modules/`, `app/ui/build/`, `.svelte-kit/`.
  - `git init -b main`, then `git add -A && git status --porcelain`.
- **Done when:** `git status --porcelain` lists no `.gba`/ROM folder; `git check-ignore "Pokemon - Emerald Version (USA, Europe)/Pokemon - Emerald Version (USA, Europe).gba"` prints the path. Initial commit `T0.0: bootstrap` allowed for this packet only.
- **Out of scope:** any crate.

### T0.1 — Model + llama-server setup script
- **Worker:** clinepass
- **Depends:** T0.0
- **Files:** create `scripts/setup-models.ps1`, `scripts/run-llm.ps1`, `models/README.md` (tracked; rest of `models/` ignored → add `!/models/README.md`).
- **Spec:**
  - Download latest llama.cpp Windows **Vulkan** x64 release zip from `github.com/ggml-org/llama.cpp/releases` → `models/llama.cpp/`. Pin the build number in the script (`$LlamaBuild = "bNNNN"`).
  - Download `unsloth/Qwen3.5-4B-GGUF`: `Qwen3.5-4B-Q4_K_M.gguf` + mmproj. If no Q8_0 mmproj is published, use F16 and record it (do NOT convert with Python).
  - Verify SHA-256 of every file against values printed by HF (`x-linked-etag`), write them into `models/README.md`.
  - `run-llm.ps1`: `llama-server -m <4B> --mmproj <mmproj> -ngl 99 -c 8192 --parallel 2 --image-max-tokens 256 --host 127.0.0.1 --port 8080 --jinja`.
  - Idempotent: skip files already present with matching hash.
- **Done when:** `pwsh scripts/setup-models.ps1` twice (2nd run downloads nothing); `pwsh scripts/run-llm.ps1` then `curl -s 127.0.0.1:8080/health` → `{"status":"ok"}`; one `/v1/chat/completions` with a base64 PNG returns text. Report VRAM used (Task Manager / `llama-server` log).
- **Out of scope:** 2B/9B (only T0.3 may fetch them, as comparison).

### T0.2 — Eval screenshot capture (EN + JA)
- **Worker:** clinepass (capture) → **HUMAN** (verify truth)
- **Depends:** T0.6 (emulator boots) — or capture with any existing emulator the user has (Eden is Switch-only; mGBA portable is acceptable for capture only)
- **Files:** `eval/screens/{en,ja}/NNN.png` (ignored), `eval/truth.jsonl` (tracked), `eval/README.md`.
- **Spec:**
  - **JA first: 40 screens** from Emerald `BPEJ` (plus 10 EN from `BPEE` as a sanity set): 20 overworld dialog, 8 battle text, 6 menu/bag/summary, 6 naming/Pokédex. Native 240×160 PNG, no filtering.
  - `truth.jsonl` line: `{"id":"en/001","game":"BPEE","kind":"dialog","text":["line1","line2"],"notes":""}`. EN text can be cross-checked against pret/pokeemerald `data/text` (reference only, do not copy files into repo beyond the per-screen truth strings). JA: worker drafts, **user verifies** every line (kana/kanji exactness matters).
- **Done when:** 80 entries, every `id` has a PNG, `eval/README.md` lists who verified JA.
- **Out of scope:** German (dropped).

### T0.3 — OCR + translation eval harness
- **Worker:** clinepass
- **Depends:** T0.1, T0.2
- **Files:** `spikes/eval-ocr/` (Rust bin crate, not in workspace members yet), `eval/report-ocr.md`.
- **Spec:**
  - For each screen: variants `raw` and `nearest 4×`. Prompt (system): "Read all text in this Game Boy Advance screenshot exactly. Return JSON." Use `response_format`/`json_schema` `{lines:[{text}], source_lang}`; temperature 0; thinking off (empty `<think></think>` via chat template / `chat_template_kwargs: {"enable_thinking": false}`).
  - Metrics: CER per language (Levenshtein on chars, NFC-normalized, whitespace collapsed), JSON-valid rate, p50/p95 latency.
  - Translation pass on correct OCR text → `id` and `en`; 10 samples per language scored by user 1–5 (put a blank score column in the report).
  - Optional comparison row: Qwen3.5-2B and -9B (download in this packet only, delete after).
- **Done when:** `cargo run --release --manifest-path spikes/eval-ocr/Cargo.toml -- --lang en` and `--lang ja` produce the table in `eval/report-ocr.md`.
- **Gate:** 4B with 4× → CER en ≤ 2 %, ja ≤ 5 %, p50 ≤ 1.8 s. If missed, report which variant/model passes.

### T0.4 — Dedicated-OCR comparison (only if T0.3 gate fails for JA)
- **Worker:** clinepass
- **Depends:** T0.3
- **Spec:** PP-OCRv5 ONNX (det+rec, ja/en) via `ort` crate, then 4B text-only translation. Same metrics, same report. Skipped entirely if T0.3 passes → write "skipped, 4B passed" in the report.

### T0.5 — Voice spike
- **Worker:** clinepass (measure) → **HUMAN** (listen)
- **Depends:** T0.0
- **Files:** `spikes/voice/`, `eval/report-voice.md`, model downloads appended to `scripts/setup-models.ps1`.
- **Spec:** whisper-rs + Whisper `small` (ggml q5_1), 10 own-recorded clips per language (ja first, then en; user records via a `record` subcommand using cpal, 16 kHz mono WAV in `eval/voice/` ignored); energy VAD (RMS threshold + hangover 300 ms) vs whisper.cpp built-in VAD; sherpa-rs Kokoro int8 (ja, en): RTF on 10 sentences/lang. Try whisper-rs `vulkan` feature; fall back to CPU and record numbers.
- **Done when:** report has WER, STT latency, RTF table; user fills "naturalness 1–5" per TTS voice.
- **Gate:** STT ≤ 400 ms for ≤ 3 s utterance, TTS RTF < 0.2, user naturalness ≥ 3 for each language. Kokoro-ja < 3 → note alternative to test (sherpa-onnx VITS ja).

### T0.6 — Emulator core spike
- **Worker:** clinepass
- **Depends:** T0.0
- **Files:** `spikes/emu/`, `docs/adr/0002-emulator.md`.
- **Spec:** rustboyadvance-ng as a **git dependency pinned to a commit** (verify license file = MIT and record it). Headless: load ROM from `OEMU_ROM_JA` (primary; repeat with `OEMU_ROM_EN`), run 3600 frames, dump frame 3600 PNG, print fps. Then with a window (`minifb` ok for spike): intro → New Game → first Prof. Birch dialog, save in-game, reload. BIOS: open-source replacement BIOS (record source + license) and, if user supplies, real BIOS via `OEMU_BIOS`. Test both `BPEE` and `BPEJ`.
- **Done when:** headless ≥ 300 fps (so 60 fps leaves headroom), in-game save survives restart (Flash 128K), RTC readable. Otherwise test mGBA (MPL-2.0) via FFI and document.
- **Out of scope:** audio quality, input mapping beyond keyboard.

### T0.7 — ADR: pipeline decision
- **Worker:** free_pi (writes) → Hermes review
- **Depends:** T0.3–T0.6
- **Files:** `docs/adr/0001-pipeline.md`.
- **Spec:** One page: chosen OCR path, model list (expected: exactly 4B + Whisper small + Kokoro), whether 2B/9B are enabled and why (numbers from reports), emulator core, open risks.
- **Done when:** every claim cites a row in `eval/report-*.md`.
