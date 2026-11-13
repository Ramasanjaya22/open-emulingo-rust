# Open-Emulingo (Rust, offline) — Reverse-Engineering & Implementation Plan

> **For Hermes:** Plan only. Eksekusi per fase; Fase 0 (spike) wajib lulus sebelum Fase 1+. Detail TDD per-task di-expand saat fase dimulai.

**Goal:** Clone fungsional (clean-room) dari emulingo.com — belajar bahasa sambil main game GBA — yang jalan **100% offline** di Windows, ditulis dengan Rust, pakai model open-source seminimal mungkin (satu VLM Qwen3.5-4B + voice kecil), untuk belajar **Japanese (fokus utama)** dan English (sekunder) dengan terjemahan ke **Indonesian / English** (opsi).

**Architecture:** Desktop app Tauri 2 (UI SvelteKit + TS) di atas Rust core (Cargo workspace). Emulator GBA, frame analysis, SRS, storage, dan voice pipeline jalan in-process di Rust; inference LLM/VLM lewat sidecar `llama-server` (llama.cpp, Vulkan untuk RX 9060 XT) di `127.0.0.1`. Tidak ada request network keluar.

**Tech Stack:** Rust (stable), Tauri 2, SvelteKit + TypeScript + pnpm (adapter-static), llama.cpp `llama-server` (Vulkan), Qwen3.5-4B GGUF + mmproj (2B/9B opsional), whisper.cpp (`whisper-rs`), sherpa-onnx (`sherpa-rs`: Kokoro), `cpal`, `rusqlite`, `fsrs`, `lindera`.

---

## 1. Hasil recon: fitur Emulingo (sumber publik)

Sumber: landing page, `/legal`, `/privacy` (diakses 2026-11-12). Clean-room: hanya deskripsi perilaku publik — **tidak** mengcopy kode, JS bundle, aset, atau brand.

| # | Fitur Emulingo | Cara mereka (dari privacy policy) | Versi kita (offline) |
|---|---|---|---|
| F1 | **Emulator GBA in-browser**, user load ROM sendiri, ROM tidak di-upload | gbajs (Jeffrey Pfau, BSD) via react-gbajs; ROM di browser storage; kenali game via SHA-256 + title/game code header cartridge | Emulator Rust native; identifikasi game via SHA-256 + header ROM (0xA0 title, 0xAC game code) |
| F2 | **Translate any screen** (manual) | Screenshot → server → OpenRouter → **Gemini**: baca teks + translate. Hasil: original text, **reading**, translation | Screenshot → VLM lokal (Qwen3.5) → JSON terstruktur (text, reading, translation, tokens) |
| F3 | **Auto-translate** (permintaan user) | Tidak eksplisit di policy; tiap translate = screenshot | Deteksi dialog-box berubah → trigger translate otomatis, cache by hash |
| F4 | **Translation history** | Disimpan di akun cloud, sync | SQLite lokal |
| F5 | **Vocabulary + review schedule (SRS), quest, study days, XP** | Cloud | SQLite + FSRS |
| F6 | **Ask about a line** (tanya grammar/arti) | Text → AI provider | LLM lokal 4B/9B, konteks = baris + layar |
| F7 | **Reader + word lookup** | Text → AI provider | Kamus offline (JMdict; kaikki untuk English) + LLM fallback |
| F8 | **Read aloud (TTS)** | OpenAI TTS | Kokoro (ja/en), lokal |
| F9 | **Companion characters** (nama, personality, avatar crop dari layar, memory tentang user) | OpenRouter (Gemini/Mistral); chat disimpan di device | Agent LLM lokal + memory SQLite |
| F10 | **Real-time voice call dengan companion** | OpenAI Realtime; STT OpenAI; caption via Web Speech API | Pipeline lokal VAD → STT → LLM streaming → TTS, barge-in |
| F11 | **Pronunciation practice** | Mic → OpenAI STT | Whisper lokal + skor kemiripan vs target |
| F12 | **Second screen pairing** (HP tampil translation/review, kode pairing) | Lewat database cloud | LAN WebSocket lokal (axum) + kode pairing, tanpa internet |
| F13 | **Save states + cloud sync** | Firebase storage | Save state lokal (sync = out of scope) |
| F14 | **Rating/koreksi translation**, export backup | Cloud | Koreksi disimpan lokal → jadi eval set; export JSON |
| — | Akun, Stripe, Firebase, Meta Pixel, GA, demo cartridges | — | **Di-drop** (offline, no tracking, no ROM distribution) |

Insight teknis kunci: Emulingo **tidak membaca RAM game** — mereka OCR via VLM dari screenshot. Jadi seluruh fitur bisa direplikasi generik untuk semua game tanpa per-game hack.

---

## 2. Model stack — seminimal mungkin (keputusan user 2026-11-12)

Prinsip: **satu model teks/gambar untuk semua fitur**, model voice hanya yang tidak bisa digantikan. Hardware: RX 9060 XT **16 GB VRAM**, Windows saja.

### Default set (3 model, ±3 GB total di disk)

| # | Fungsi | Model | ~Size/VRAM | Catatan |
|---|---|---|---|---|
| 1 | OCR + translate + explain + companion agent | **Qwen3.5-4B** Q4_K_M + mmproj Q8_0 | ~2.6 GB VRAM | Satu-satunya LLM. Resident terus |
| 2 | STT | Whisper `small` q5 (whisper.cpp) | ~190 MB | ja, en. VAD: energy-based di Rust (tanpa model); whisper.cpp built-in Silero VAD hanya jika energy VAD gagal di spike |
| 3 | TTS ja + en | Kokoro-82M int8 (sherpa-onnx) | ~90 MB | ja "partial" (misaki) — diuji di spike |

### Opsional — hanya diaktifkan kalau spike membuktikan 4B kurang
- **2B** (`Qwen3.5-2B`): hanya jika auto-translate 4B > 1.8 s.
- **9B** (`Qwen3.5-9B`): hanya jika kualitas OCR kanji / companion 4B tidak lulus eval. Dengan 16 GB, 9B (~7 GB) + 4B bisa resident bersamaan — tidak perlu swap.

`crates/llm` tetap punya enum `Tier`, tapi default konfigurasi = satu model untuk semua tier.

Catatan dari benchmark publik: mmproj **Q8_0 + `--image-max-tokens 256`** memangkas encoding ~50% tanpa turun kualitas; mmproj di bawah Q8_0 merusak vision. Thinking dimatikan (`<think></think>` kosong) untuk latency.

Semua voice dimuat via `sherpa-rs` / `whisper-rs` (FFI ke C/C++) — tidak ada Python runtime (constitution: Python default-deny).

### Arah bahasa
- **Bahasa yang dipelajari (sumber teks game):** **Japanese (fokus, semua eval gate)**, English (sekunder). German di-skip (keputusan user 2026-11-12).
- **Bahasa terjemahan (opsi user di Settings):** **Indonesian** atau **English** (default Indonesian).
- Kasus sumber = English & target = English → mode "simplify + explain" (parafrase sederhana + definisi), bukan terjemah.

### NLP pendukung (tanpa model)
- Japanese tokenize + furigana: `lindera` (IPADIC/UniDic).
- Kamus offline: JMdict (ja→en), kaikki.org Wiktionary extract (en). Lisensi CC-BY-SA → tampilkan atribusi.
- Gloss ke **Indonesian**: tidak ada kamus offline ja→id yang layak → gloss dibuat oleh 4B sekali per lemma, lalu di-cache di SQLite (jadi kamus id tumbuh sendiri).

---

## 3. Arsitektur

```
open-emulingo-rust/
├─ Cargo.toml                 # workspace
├─ crates/
│  ├─ emu/        # trait Emulator + adapter rustboyadvance-ng; frame RGBA, input, audio, save state
│  ├─ screen/     # frame diff, dialog-box detection, dHash, upscale nearest 4x, PNG encode
│  ├─ llm/        # client OpenAI-compatible ke llama-server; tier routing 2B/4B/9B; json_schema output
│  ├─ translate/  # prompt, parse JSON hasil, cache by region hash, pipeline auto/manual
│  ├─ lang/       # lindera (ja), kamus lookup, lemma, normalisasi teks
│  ├─ srs/        # FSRS scheduling, XP, streak, quest sederhana
│  ├─ voice/      # cpal I/O, VAD, STT whisper, TTS kokoro, sentence chunker
│  ├─ agent/      # companion: persona, memory, state machine real-time, barge-in
│  ├─ store/      # rusqlite schema + migrations
│  └─ pair/       # (Fase 6) axum LAN WebSocket second screen
├─ app/           # Tauri 2 shell (src-tauri) + SvelteKit UI (ui/)
├─ models/        # .gitignore — GGUF/ONNX, di-download oleh script setup
└─ eval/          # dataset screenshot + ground truth (TIDAK berisi ROM)
```

Keputusan & alasan:
- **llama-server sidecar, bukan embed `llama-cpp-2`** — prebuilt Vulkan build langsung jalan di RX 9060 XT, crash inference tidak menjatuhkan emulator, swap model tanpa rebuild. Komunikasi HTTP lokal (OpenAI-compatible + `json_schema`). Embed bisa dipertimbangkan nanti kalau overhead terukur.
- **Emulator: `rustboyadvance-ng`** (pure Rust) di balik trait `Emulator`; fallback ke mGBA via FFI kalau akurasi game target buruk. BIOS: pakai open-source replacement BIOS (mis. Cult-of-GBA BIOS) supaya tidak butuh dump BIOS Nintendo; user boleh pilih BIOS asli miliknya.
- **Tauri + SvelteKit** — default stack constitution; UI ikut `clean-apple-design` (light mode default). Frame 240×160 RGBA (~150 KB) di-stream ke canvas via Tauri `Channel` binary @60fps (~9 MB/s). Risiko → lihat §7.
- **SQLite** sah di sini: produk memang lokal/embedded.
- **Tidak ada network keluar.** Model di-download sekali oleh script setup; runtime menolak host non-loopback.

### Interface inti (kontrak antar crate)

```rust
// crates/emu
pub trait Emulator {
    fn load_rom(&mut self, rom: &[u8]) -> Result<RomInfo>;
    fn run_frame(&mut self) -> &Frame;            // 240x160 RGBA8
    fn set_keys(&mut self, keys: KeyState);
    fn drain_audio(&mut self, out: &mut Vec<i16>);
    fn save_state(&self) -> Vec<u8>;
    fn load_state(&mut self, s: &[u8]) -> Result<()>;
}
pub struct RomInfo { pub sha256: [u8; 32], pub title: String, pub game_code: String }

// crates/translate — output terstruktur dari VLM (dipaksa via json_schema)
pub struct ScreenTranslation {
    pub source_lang: Lang,                // Ja | En
    pub lines: Vec<Line>,
}
pub struct Line { pub text: String, pub reading: Option<String>, pub translation: String, pub tokens: Vec<Token> }
pub struct Token { pub surface: String, pub lemma: String, pub reading: Option<String>, pub pos: String, pub gloss: String }

// crates/llm
pub enum Tier { Fast2B, Default4B, Agent9B }
```

---

## 4. Desain fitur utama

### 4.1 Auto-translate (F3)
1. Tiap frame: hitung hash per region kandidat (default: 1/3 bawah layar = dialog box GBA umum; user bisa drag region custom per game, disimpan by `game_code`).
2. Text **stabil** = hash region tidak berubah selama N=12 frame (~200 ms) setelah berubah (menangkap akhir animasi typewriter).
3. Cache lookup `(game_code, dHash region)` di SQLite → hit = tampil instan (0 inference).
4. Miss → upscale nearest 4× → PNG → **4B** VLM, `temperature 0`, `json_schema`, bahasa target dari Settings (id/en). Debounce: maksimal 1 request in-flight; request lama dibatalkan jika teks baru muncul.
5. Hasil masuk history + kandidat vocab. Tombol "refine" → re-run dengan prompt lebih detail (atau 9B bila diaktifkan).

Target latency: ≤ 1.8 s (4B) dari teks stabil sampai overlay muncul, di RX 9060 XT. Kalau gagal → aktifkan 2B (lihat §2).

Catatan Pokémon Emerald: dialog box di 2 baris bawah (y≈112–160), teks muncul per karakter dan menunggu tombol A (▼) — sinyal "stabil" = indikator ▼ muncul / hash tetap. Menu/battle text di region berbeda → preset region per `game_code` (`BPEJ` utama, `BPEE`).

### 4.2 Manual translate + word breakdown (F2, F7)
Hotkey → freeze frame → 4B VLM → overlay per baris; klik token → kamus offline (lindera reading + JMdict (ja) / kaikki (en)); bila tidak ada → gloss dari LLM. "Add to deck" → FSRS card.

### 4.3 Real-time companion agent (F9, F10)
State machine: `Idle → Listening (VAD) → Transcribing → Thinking (stream) → Speaking → Idle`, dengan **barge-in**: VAD aktif saat Speaking → stop TTS, kembali ke Listening.

```
mic(cpal 16k) → energy VAD → Whisper small (lang hint = bahasa target)
   → prompt[persona + memory facts + screen text terkini + 6 turn terakhir]
   → 4B streaming tokens (9B jika diaktifkan) → sentence chunker (。！？.!?) → Kokoro → speaker
```
- Persona: nama, personality, avatar (crop region frame, PNG di SQLite).
- Memory: setelah sesi, 4B merangkum → fakta pendek (`user suka X`, `kesulitan partikel は/が`) disimpan; di-inject top-k by recency.
- Level-aware: companion bicara di bahasa target dengan kosakata sesuai level user; koreksi lembut.
- Target: first audio ≤ 1.5 s setelah user selesai bicara (VAD end ~300 ms + STT ~300 ms + TTFT ~400 ms + TTS chunk pertama ~200 ms).
- Model sama (4B) dipakai translate & companion → request translate diprioritaskan; llama-server `--parallel 2` agar keduanya tidak saling blok.

### 4.4 SRS & progres (F5)
`fsrs` crate; kartu = lemma + contoh kalimat asli dari game + screenshot crop. XP per translate/review, streak harian, quest sederhana (mis. "review 20 kartu", "translate 10 dialog").

### 4.5 Pronunciation (F11)
TTS putar target → user ucapkan → Whisper (lang dipaksa) → skor = 1 − normalized edit distance (kana untuk ja via lindera, lowercase untuk en). Bukan phoneme-level di v1 (YAGNI).

### 4.6 Second screen (F12, Fase 6)
`axum` di LAN, kode pairing 6 digit, WebSocket push translation/review/companion; halaman web statis dilayani oleh app itu sendiri. Hanya listen di interface LAN bila user mengaktifkan.

---

## 5. Fase & task

> Ringkasan saja. Task packet yang bisa langsung didelegasikan (worker, files, spec, done-when) ada di `docs/plan/phase-*.md`; aturan delegasi free-pi/ClinePass di `docs/plan/01-delegation.md`.

### Fase 0 — Spike (go/no-go, ~2–3 hari) — WAJIB duluan
| Task | Output | Lulus jika |
|---|---|---|
| 0.1 Setup llama.cpp Vulkan prebuilt (Windows), download Qwen3.5-4B Q4_K_M + mmproj Q8_0 (2B/9B hanya untuk pembanding) | `scripts/setup-models.ps1` | `llama-server --mmproj` jawab request gambar |
| 0.2 Eval set dari **Pokémon Emerald** milik user: 40 screenshot (dialog, menu, battle, nama Pokémon) + ground truth teks manual. ROM JP (`BPEJ`) jadi sumber utama; EN (`BPEE`) untuk smoke test | `eval/screens/*.png` + `eval/truth.jsonl` | ≥ 40 sampel JA; ≥ 10 EN |
| 0.3 Ukur OCR CER, kualitas terjemahan (→id, →en) & latency 4B (raw vs upscale 4×); 2B/9B sebagai pembanding | `eval/report-ocr.md` | 4B CER ja ≤ 5%, en ≤ 2%, latency ≤ 1.8 s → 2B/9B tidak dipakai |
| 0.4 Bandingkan vs PP-OCRv5 (ONNX via `ort`) + text-only LLM | tabel di report | pilih pipeline pemenang |
| 0.5 Voice: whisper small ja/en WER pada 10 klip; energy VAD vs Silero; Kokoro ja (utama) + en kualitas & RTF | `eval/report-voice.md` | RTF TTS < 0.2, STT < 400 ms/utterance pendek |
| 0.6 Emulator: rustboyadvance-ng boot Emerald (`BPEE`, SHA-256 `a9dec84d…085af`) dengan replacement BIOS, intro → New Game → dialog Prof. Birch, 60 fps | log fps | ≥ 60 fps stabil, save in-game (Flash 128K + RTC) berfungsi; jika gagal → mGBA FFI |

Keputusan setelah Fase 0 ditulis di `docs/adr/0001-pipeline.md`.

### Fase 1 — Core emulator + app shell
1. `cargo new --lib` workspace + crates `emu`, `store`; CI lokal `cargo clippy -D warnings && cargo test`.
2. `emu`: trait + adapter, test: `RomInfo` parsing header dari ROM homebrew kecil di `tests/fixtures` (homebrew berlisensi bebas).
3. `store`: schema v1 (`games`, `translations`, `cards`, `reviews`, `companions`, `memories`, `settings`) + migration test.
4. Tauri app: load ROM (file dialog), canvas render, keyboard/gamepad input, audio, save/load state.
   - Verifikasi: `cargo tauri dev`, main 5 menit tanpa drop frame (fps counter overlay).

### Fase 2 — Manual translate
1. `screen`: freeze, crop, upscale, PNG encode (unit test ukuran output = 960×640).
2. `llm`: client + `json_schema`; test dengan mock HTTP server (`wiremock`).
3. `translate`: prompt per bahasa sumber, parser ke `ScreenTranslation`, error → retry 1× dengan 4B.
4. UI overlay translation + history panel.
   - Verifikasi: jalankan eval set via `cargo run -p translate --example eval` → angka sama dengan Fase 0.

### Fase 3 — Auto-translate
1. dHash region + stabilitas N-frame (unit test dengan sequence frame sintetis: typewriter → stabil → trigger tepat 1×).
2. Cache SQLite + cancel in-flight.
3. Region editor per game.
   - Verifikasi: 10 menit gameplay, hitung trigger vs dialog nyata (precision ≥ 90%, tidak ada duplikat).

### Fase 4 — Kamus, vocab, SRS, XP
1. Import JMdict + kaikki ke SQLite FTS5 (`scripts/import-dicts`), lindera tokenize.
2. Word lookup popup, add-to-deck, review screen FSRS, XP/streak/quest.
   - Verifikasi: test FSRS interval, test lookup `食べた → 食べる`, `行った → 行く`.

### Fase 5 — Voice & real-time companion
1. `voice`: cpal capture/playback, VAD, STT, TTS router (ja/en → Kokoro).
2. `agent`: state machine + barge-in (unit test transisi state dengan event sintetis), sentence chunker test.
3. Companion setup UI (nama, personality, crop avatar), memory summarizer.
4. Read-aloud tombol per baris + pronunciation practice.
   - Verifikasi: ukur latency end-to-end 20 turn → p50 ≤ 1.5 s; barge-in menghentikan audio < 150 ms.

### Fase 6 — Polish
Second screen LAN, export/import backup JSON, koreksi user → `eval/`, model manager (pilih tier, cek VRAM), packaging installer Windows (`cargo tauri build`), atribusi lisensi (JMdict, Wiktionary, model licenses, emulator).

---

## 6. Validasi global
- `cargo fmt --check && cargo clippy --workspace -- -D warnings && cargo test --workspace` hijau; cek `cargo test -- --list` untuk memastikan semua test terdaftar.
- Eval regresi translate (`eval/`) dijalankan tiap ganti prompt/model; simpan angka di `eval/history.csv`.
- UI diverifikasi dengan Obscura (bukan Playwright/Chromium) untuk halaman second-screen; app Tauri via skill `verifying-tauri-webview2-ui`.
- Uji offline: jalankan app dengan network adapter dimatikan — semua fitur harus jalan.

## 7. Risiko & mitigasi
| Risiko | Mitigasi |
|---|---|
| VLM kecil salah baca font pixel Jepang (kanji kecil) | Upscale 4× nearest; 4B/9B untuk refine; Fase 0.4 OCR dedicated sebagai alternatif; koreksi user → few-shot per game |
| 4B tidak cukup untuk kanji kecil / companion | Aktifkan 9B opsional (16 GB VRAM muat 4B+9B resident); keputusan berbasis angka eval, bukan feeling |
| Emerald butuh Flash 128K + RTC (berry/tide) | Uji di Fase 0.6; RTC dari jam sistem |
| IPC frame Tauri 60fps berat | Channel binary + `requestAnimationFrame`; fallback render via `wgpu` native window / egui jika jank |
| Kokoro Japanese "partial" | Fase 0.5 dengar langsung; alternatif sherpa-onnx VITS ja / MeloTTS ONNX |
| rustboyadvance-ng kompatibilitas game | Trait `Emulator` → swap ke mGBA FFI (MPL-2.0) |
| Legal | Tidak distribusi ROM/BIOS Nintendo; clean-room (tanpa copy kode/aset Emulingo); **nama produk jangan mengandung "Emulingo"** (trademark) — `open-emulingo` hanya nama kerja |

## 8. Keputusan (user, 2026-11-12) & sisa pertanyaan
Diputuskan: terjemahan ke **Indonesian + English sebagai opsi**; VRAM **16 GB**; model open-source **seminimal mungkin**; **Windows dulu**; game eval **Pokémon Emerald** (`F:\@workspace\open-emulingo-rust\Pokemon - Emerald Version (USA, Europe)\`, game code `BPEE` = versi English).

Sisa:
1. ROM JP **tersedia**: `Pokemon Emerald (J)(Independent)/1649 - Pokemon Emerald (J)(Independent).gba` (`BPEJ`, SHA-256 `33f5610b…6343c`). German di-skip → tidak butuh ROM DE.
2. Folder ROM ada di root project → wajib masuk `.gitignore` (`*.gba`, `Pokemon*/`) sebelum `git init`, supaya ROM tidak pernah ter-commit.
