# Phase 4–6 — Dictionary & SRS, voice & companion, polish

---

## Phase 4 — Dictionary, vocab, SRS, XP

### T4.1 — Dictionary import tool
- **Worker:** clinepass
- **Depends:** T1.3
- **Files:** `crates/lang/src/import.rs`, `crates/lang/examples/import_dicts.rs`, `scripts/setup-dicts.ps1`.
- **Spec:** download JMdict_e (XML) and kaikki.org Wiktionary JSONL extract for English (headwords with English glosses) into `models/dicts/`; import into a separate SQLite `models/dicts/dict.sqlite` with FTS5 table `entries(lemma, reading, lang, pos, gloss_en)`. Streaming parse (`quick-xml`, `serde_json` line-by-line); no full-file in RAM. Record source URLs, dates, licenses (CC-BY-SA) in `models/dicts/ATTRIBUTION.md` (tracked).
- **Done when:** import finishes; `examples/import_dicts -- --query 食べる` and `--query run` return rows; import time + db size in report.

### T4.2 — `lang` crate: tokenize + lookup
- **Worker:** free_pi
- **Depends:** T4.1
- **Files:** `crates/lang/src/{lib.rs,ja.rs,lookup.rs}`, tests.
- **Spec:** `lindera` (embedded IPADIC) for ja → surface, base form, reading (katakana → convert to hiragana). `lookup(lemma, lang, target_lang)`: order = `lexicon` cache → dict.sqlite (en glosses) → `None` (caller asks LLM). Target `id`: dict gives en gloss, then the LLM gloss to id is requested by caller and written into `lexicon` with source `llm`.
- **Tests:** `食べた` → lemma `食べる`, reading `たべた`; `行った` → lemma `行く`; English `ran` → `run` via kaikki forms else `None` (document); cache hit returns without touching dict.
- **Done when:** green (dict-dependent tests `#[ignore]`).

### T4.3 — `srs` crate: FSRS + XP + streak + quests
- **Worker:** free_pi
- **Depends:** T1.3
- **Files:** `crates/srs/**`, tests.
- **Spec:** `fsrs` crate default parameters; `Card` ↔ `cards.fsrs_state` JSON; `review(card, Rating, now) -> Card`; `due(now, limit)`. XP: +1 per translation viewed, +2 per review, +5 per new card. Streak = consecutive `stats.day` with xp > 0 (local date). Daily quests (static list): "review 20 cards", "translate 10 lines", "add 5 words" → progress from `stats`.
- **Tests:** good rating pushes due date later than hard; streak across midnight; quest progress counts.
- **Done when:** green.

### T4.4 — UI: word popup, deck, review
- **Worker:** clinepass
- **Depends:** T4.2, T4.3, T2.4
- **Spec:** token popup shows dict result (or "ask AI" → LLM gloss, cached); "Add to deck" stores lemma + example line + 4× crop of the dialog box. Review screen: card front (word + game crop), reveal, 4 rating buttons. Header: XP, streak, quests.
- **Done when:** builds; **HUMAN**: add 5 words from Emerald JA, review them.

---

## Phase 5 — Voice & real-time companion

### T5.1 — `voice` crate: audio I/O + energy VAD
- **Worker:** free_pi
- **Depends:** T0.7
- **Files:** `crates/voice/src/{vad.rs,chunker.rs}`, tests.
- **Spec:** pure logic only: `EnergyVad { threshold_db, start_ms: 120, hangover_ms: 300 }` over 16 kHz i16 frames of 20 ms → events `SpeechStart`/`SpeechEnd`. `SentenceChunker::push(&str) -> Vec<String>` splits streamed LLM text at `。！？.!?` + newline, keeps abbreviations like "Mr." / "e.g." together, flushes remainder on `finish()`.
- **Tests:** synthetic sine/silence buffers; chunker with tokens split mid-sentence; ja + en strings.
- **Done when:** green.

### T5.2 — `voice` crate: STT + TTS + device I/O
- **Worker:** clinepass
- **Depends:** T5.1, T0.5
- **Files:** `crates/voice/src/{stt.rs,tts.rs,io.rs}`.
- **Spec:** `Stt::transcribe(pcm:&[f32], lang:Lang) -> String` (whisper-rs, model loaded once, Vulkan if T0.5 says it works). `Tts::speak(text, lang) -> Vec<f32>` routes ja/en → Kokoro (sherpa-rs). `io`: cpal input (resample → 16 kHz mono) and output with `stop()` that silences within 50 ms.
- **Done when:** `#[ignore]` tests with model paths from env pass; `examples/say.rs "こんにちは" ja` plays audio.

### T5.3 — `agent` crate: companion state machine
- **Worker:** clinepass
- **Depends:** T5.1, T2.2
- **Files:** `crates/agent/**`.
- **Spec:** states `Idle → Listening → Transcribing → Thinking → Speaking → Idle`; barge-in: `SpeechStart` while `Speaking` → stop TTS, cancel LLM stream, go `Listening`. Prompt = persona + level + top-8 memories + current screen text (latest translation) + last 6 turns; reply in the game language, short (≤ 2 sentences), gentle correction of user mistakes, glosses in target lang on request. After a session: summarize into ≤ 5 facts → `memories`. All side effects behind traits (`SttPort`, `TtsPort`, `LlmPort`, `Clock`) so the machine is unit-testable.
- **Tests:** scripted event sequences with fakes: normal turn; barge-in during Speaking; LLM error → back to Idle with message; first TTS chunk sent before LLM stream ends.
- **Done when:** green; latency log fields present (vad_end, stt_done, first_token, first_audio).

### T5.4 — UI: companions, voice call, read-aloud, pronunciation
- **Worker:** clinepass
- **Depends:** T5.2, T5.3, T2.4
- **Spec:** create companion (name, personality text, crop avatar from paused frame); push-to-talk + hands-free call mode with live caption (whisper partials or final only — no browser speech API, offline); speaker button per translated line; pronunciation drill: play line → record → score = 1 − normalized edit distance (ja compared in hiragana via `lang`).
- **Done when:** builds; **HUMAN**: 20 voice turns in JA, p50 vad_end→first_audio ≤ 1.5 s from logs; barge-in stops audio immediately.

---

## Phase 6 — Polish

| Id | Worker | Task | Done when |
|---|---|---|---|
| T6.1 | clinepass | Second screen: `crates/pair` axum on LAN only when enabled, 6-digit code, WebSocket push of translations/review/companion text, static page served by the app | phone browser on same Wi-Fi shows live translations; disabled = no listening socket (`netstat`) |
| T6.2 | free_pi | Export/import backup JSON (cards, reviews, translations, companions, settings; no ROMs/saves) | round-trip test equal |
| T6.3 | free_pi | User corrections → `eval/corrections.jsonl`; rerun eval includes them | eval example reads file |
| T6.4 | clinepass | Model manager screen: shows which models are present/missing, VRAM in use, start/stop llama-server sidecar from the app | start/stop works without terminal |
| T6.5 | clinepass | Windows installer `cargo tauri build` (NSIS), first-run wizard: pick BIOS (optional), run model + dict setup | clean Windows user profile: install → setup → play → translate offline (network adapter disabled) |
| T6.6 | free_pi | `THIRD_PARTY.md`: licenses for emulator core, BIOS replacement, models (Qwen, Whisper, Kokoro), dicts, crates (`cargo about` or `cargo license` output) | file covers every dependency |
| T6.7 | **HUMAN** | Pick product name (not containing "Emulingo") | name in README |
