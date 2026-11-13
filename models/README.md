# Models (local only)

Model binaries are **not** committed: `/models/*` is gitignored except this file.
Everything here is fetched by `scripts/setup-models.ps1`, which verifies each SHA-256 and
re-fetches anything that does not match. At runtime only `127.0.0.1` is contacted.

## Pinned set

Pinned 2026-11-13. Bump `$LlamaBuild` in `scripts/setup-models.ps1` and the two hashes below
together; nothing else references the build number.

| File | Bytes | SHA-256 | Source |
|---|---|---|---|
| `llama.cpp/llama-b12344-bin-win-vulkan-x64.zip` | 38104929 | `c64f8e79bdc877065bfefb45fe292c0fee5cbf1a0342cb9c728031dc51d548ed` | `github.com/ggml-org/llama.cpp/releases/download/b12344/llama-b12344-bin-win-vulkan-x64.zip` |
| `Qwen3.5-4B-GGUF/Qwen3.5-4B-Q4_K_M.gguf` | 2740937888 | `00fe7986ff5f6b463e62455821146049db6f9313603938a70800d1fb69ef11a4` | `huggingface.co/unsloth/Qwen3.5-4B-GGUF` (repo `e87f1764`) |
| `Qwen3.5-4B-GGUF/mmproj-F16.gguf` | 672423616 | `cd88edcf8d031894960bb0c9c5b9b7e1fea6ebee02b9f7ce925a00d12891f864` | `huggingface.co/unsloth/Qwen3.5-4B-GGUF` (repo `e87f1764`) |

Hashes are the values Hugging Face reports as the LFS `x-linked-etag` for each file and the
GitHub release asset digest for the zip.

### mmproj is F16, not Q8_0

`unsloth/Qwen3.5-4B-GGUF` publishes `mmproj-BF16`, `mmproj-F16` and `mmproj-F32` only — there is
no Q8_0 projection file. F16 is used (672 MB); conversion is out of scope and would need Python,
which this repo forbids.

## Usage

```powershell
pwsh scripts/setup-models.ps1     # download + verify, safe to re-run
pwsh scripts/run-llm.ps1          # llama-server on http://127.0.0.1:8080
curl.exe -s 127.0.0.1:8080/health # {"status":"ok"} once the model is loaded
```

`scripts/run-llm.ps1` runs in the foreground; Ctrl+C stops it. Model load on the RX 9060 XT
takes roughly 10-20 s before `/health` reports `ok`.
