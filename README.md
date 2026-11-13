# OpenKotoba

> **Learn Japanese by Playing.** An offline, privacy-first desktop application to play GBA games, translate on-screen dialogue via local LLMs, and master vocabulary naturally — without sending data to any cloud.

<p align="center">
  <img src="docs/marketing/visuals/logo-lockup@2x.png" alt="OpenKotoba Logo" width="700">
</p>

---

## High-Fidelity Interface Previews

### 1. In-Game Screen Translation & Live Vocabulary Extract
Playing Game Boy Advance titles with real-time translation and parsed vocabulary lists:

<p align="center">
  <img src="docs/marketing/visuals/shot-play@2x.png" alt="OpenKotoba Translation Overlay" width="900">
</p>

### 2. Speaking Practice & Conversational Companion
Interactive voice-assisted practice with local Kokoro TTS and gentle corrections:

<p align="center">
  <img src="docs/marketing/visuals/shot-companion@2x.png" alt="OpenKotoba Speaking Companion" width="900">
</p>

---

## Core Architecture
- **Runtime:** Tauri 2 + SvelteKit + Rust workspace. 100% offline at runtime.
- **Inference:** Local `llama-server` (Vulkan) running Qwen3.5-4B + mmproj for live screen OCR/translation.
- **Voice:** whisper-rs and sherpa-rs (Kokoro) for local speech recognition and synthesis.
- **SRS & Parsing:** rusqlite bundled storage, FSRS spaced repetition, and Lindera morphological analyzer.

Plan: `docs/plan/README.md`. Agent rules: `AGENTS.md`.
