# 01 — Delegation: free-pi + ClinePass

Who implements: **free-pi** and **ClinePass** only (user decision 2026-11-12).
Who plans/reviews: **Hermes (brain)** — writes no project code; dispatches a task packet, reviews the result, accepts or sends feedback.

> Project override of `~/agents/routing.yaml`: there free_pi is only search/summarize/file_ops. For this project free_pi also implements **single-crate, well-specified** tasks; ClinePass implements **multi-file / FFI / UI / integration** tasks.

## Routing rule

| Task kind | Worker | Examples |
|---|---|---|
| One crate, pure logic, unit tests decide pass/fail, ≤ ~300 LOC | **free_pi** | header parser, dHash, FSRS wrapper, sentence chunker, SQLite schema, JSON parser, docs, eval scripts |
| Several crates or app, FFI/native libs, GPU/audio, Tauri/Svelte UI, judgment on design | **clinepass** | emulator adapter, Tauri shell, frame streaming, voice FFI, agent loop, llama-server integration |
| Needs eyes/ears of a human | **HUMAN** (user) | ground-truth transcription check, TTS listening test, gameplay feel |

Fallback: free_pi fails twice on a task → move it to clinepass. clinepass fails twice → stop and ask the user. Never loop silently.

Each packet in `docs/plan/phase-*.md` already states its worker in `Worker:`.

## Dispatch prompt template (paste as-is, fill `<>`)

```
You are working in F:/@workspace/open-emulingo-rust.
Read AGENTS.md first and obey it.
Implement ONLY task <T1.3> from docs/plan/<phase-1-core.md>. Read docs/plan/00-master-plan.md sections <§3> only if the packet points there.
Work on git branch task/<T1.3> (create from main).
TDD: failing test first. Finish when every "Done when" command passes.
Do not commit. End with the AGENTS.md report (max 10 lines).
```

Give the worker only that: the packet + named sections. No chat history.

## How to drive each worker (Windows host)

**free_pi** — `npx -y free-pi-cli@latest --src x` (interactive TUI, no one-shot flag; reuses existing login).
- Hermes: `terminal(background=true, pty=true, workdir="F:/@workspace/open-emulingo-rust", command="npx -y free-pi-cli@latest --src x")` → `process_manage(action="submit", data=<prompt>)` → poll output. A reply can take > 1 min; if the TUI stays on "Working" after it finished, kill and start fresh for the next packet.
- Do NOT use the bare `pi` binary on PATH (different, unauthenticated package).

**clinepass** — Cline with provider `cline-pass` (`~/.cline`), paid subscription. Use for packets marked `clinepass` only. Start it in the repo dir, paste the same template. Cline reads `AGENTS.md`.

## Review gate (Hermes, per packet)
1. `git diff --stat main...task/<id>` — only files listed in the packet changed (plus `Cargo.lock`).
2. Re-run every "Done when" command myself; compare counts with the worker's report (worker report = claim, not proof).
3. `cargo test -- --list` shows the new tests.
4. Check hard rules: no ROM/model files staged, no Python/React, no new network hosts, no unlisted deps.
5. Pass → user/Hermes merges `task/<id>` into `main` (commit message `<id>: <title>`). Fail → one feedback message with exact failing command + output, same worker.

## Order & parallelism
Packets with no shared files can run in parallel (one worker each, separate branches). Dependency lines (`Depends:`) are hard. The critical path is
`T0.* → T1.1 → T1.2 → T1.4 → T2.* → T3.*`; Phase 4 and the pure-logic parts of Phase 5 can start after T1.3.

## Tracking
One status file per active packet in `~/agents/tasks/oemu-<id>.md` (Goal / Status / Next step), per `~/agents/tasks/README.md`. Summary of finished packets: 3–5 lines in `~/agents/memory/`.
