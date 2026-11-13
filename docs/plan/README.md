# Plan index — open-emulingo-rust

Status: **planning only** (user, 2026-11-12). Nothing is executed until the user says so.
Implementation is delegated to **free-pi** and **ClinePass**; Hermes plans and reviews.

| File | Contents |
|---|---|
| `00-master-plan.md` | Emulingo feature recon, model stack, architecture, decisions, risks |
| `01-delegation.md` | Who does what, dispatch prompt template, how to drive each worker, review gate |
| `phase-0-spike.md` | T0.0–T0.7: repo bootstrap, models, eval set, OCR/voice/emulator spikes, ADR |
| `phase-1-core.md` | T1.1–T1.5: workspace, ROM header, emulator adapter, SQLite, Tauri shell, settings |
| `phase-2-3-translate.md` | T2.1–T2.4 manual translate; T3.1–T3.4 auto-translate |
| `phase-4-6.md` | T4.* dictionary/SRS, T5.* voice + companion, T6.* polish |
| `../../AGENTS.md` | Rules every worker must follow |

## Packet count by worker
| Worker | Packets |
|---|---|
| free_pi | T0.0, T0.7, T1.1, T1.3, T1.5, T2.1, T3.1, T3.2, T4.2, T4.3, T5.1, T6.2, T6.3, T6.6 (14) |
| clinepass | T0.1, T0.2, T0.3, T0.4, T0.5, T0.6, T1.2, T1.4, T2.2, T2.3, T2.4, T3.3, T3.4, T4.1, T4.4, T5.2, T5.3, T5.4, T6.1, T6.4, T6.5 (21) |
| HUMAN checkpoints | T0.2 JA truth, T0.5 listening, T1.4/T2.4/T3.3/T3.4/T4.4/T5.4 play tests, T6.7 name |

## Critical path
T0.0 → T0.1 + T0.6 → T0.2 → T0.3 → T0.7 → T1.1 → T1.2 → T1.4 → T2.3 → T2.4 → T3.3

## Decisions so far (2026-11-12)
- Learn: **ja (focus)**, en (secondary). German dropped (user, 2026-11-12). Translate into: **id or en** (setting, default id).
- Models, minimum: **Qwen3.5-4B** (only LLM/VLM) + Whisper small + Kokoro (en/ja). 2B/9B off unless eval proves 4B insufficient.
- GPU: RX 9060 XT 16 GB, Vulkan. Windows first.
- Eval game: Pokémon Emerald **JA `BPEJ` (primary)** + EN `BPEE` (emulator smoke test) — both on disk, gitignored.

## Open
1. GBA BIOS: replacement BIOS by default; does the user have a real BIOS dump to compare?
