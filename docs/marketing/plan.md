# Marketing Plan — Open-Emulingo (nama kerja, T6.7)

Status: draft, 2026-11-13. Bukan eksekusi. Angka = asumsi yang bisa diverifikasi, ditandai `[verify]`.

## 0. Baca dulu: realita produk

- **Belum ada produk.** Repo baru sampai T0.0 (bootstrap); `crates/` dan `app/` belum ada. Marketing yang jalan sekarang = janji kosong. Gerbang pertama: satu GIF 20 detik demo setelah T3.3 (auto-translate jalan di Emerald JA).
- **Scope hari-1 realistis:** emulator + manual translate + auto-translate + history. Kamus/SRS (F4), companion + voice (F5), second screen (F6) menyusul. Jangan jual F5/F6 di materi publik sebelum ada build-nya.
- **Nama belum final** (T6.7), dan `AGENTS.md`/`00-master-plan.md` §7 melarang nama mengandung "Emulingo". Kata "clone of emulingo.com" hanya boleh dipakai di tempat yang tepat (lihat §7).

## 1. Positioning

Satu kalimat:

> Belajar bahasa Jepang dari game GBA yang kamu sudah punya — terjemahan layar, kamus, dan partner bicara, jalan **100% offline** di PC Windows-mu.

Tiga klaim yang jadi tulang punggung semua materi:

1. **Offline penuh.** Kabel LAN dicabut, semua fitur tetap jalan. Ini bukan sekadar privasi, ini biaya: tidak ada langganan per-token yang membengkak.
2. **Game apa pun, tanpa hack per-game.** Deteksi teks dari piksel layar (region stabil → VLM), bukan baca RAM. Satu build mengejar semua GBA RPG/adventure, bukan cuma judul populer.
3. **GPU-mu yang kerja.** Model 4B + Whisper + Kokoro di VRAM lokal. Kompetitor mengirim screenshot ke server Gemini/OpenAI.

Slogan kandidat (pilih satu, uji di komunitas):
- "Your games. Your GPU. Zero cloud."
- "Main dulu, terjemah sambil jalan."

## 2. Audiens

Prioritas (dari kecil ke besar), sesuai beban marketing:

| # | Segmen | Kenapa mereka | Where |
|---|---|---|---|
| A | **Learner Jepang yang punya backlog GBA** (Anki/ AJATT, sudah pakai Yomitan/Rikaitan di PC) | Paling terima "vocab + SRS + immersion"; paham nilai auto-translate karena sudah bayar mahal untuk itu | r/LearnJapanese, r/Anki, Discord JP-learning (Anki, Immersion, Refold) |
| B | **Emulation/privacy crowd** (RetroArch, EmuDeck, ROM-hunter) | Reaktif alergi terhadap "cloud AI di emulator"; offline = jualan utama | r/Emulation, r/retrogaming, Libretro forum, It's-a-pixel-thing |
| C | **Indonesia: mahasiswa/pelajar Jepang & otaku** | Bahasa target default produk ini `id` — mereka dapat nilai ganda (belajar JP lewat teks Indonesia) | r/indonesia, r/Jepang, grup Discord kampus, X/Twitter JP-ID accounts |
| D | **Tech/opensource tinkerer** (Rust, local-LLM) | Distributor gratis: mereka suka fork, bikin AUR/scoop manifest, tulis blog | r/LocalLLaMA, Hacker News, Lobste.rs, r/rust |

Bukan target (dan jangan diiklanin): pemain yang mau main gratis tanpa beli ROM, dan siapa pun yang mencari "GBA emulator terbaik" — kita kalah telak dari mGBA/RetroArch di hal itu.

## 3. Pesan per segmen

| Segmen | Hook utama | Bukti yang harus ada | Bahan |
|---|---|---|---|
| A | "Yomitan untuk layar game, plus SRS-nya" | Demo: teks dialog muncul → klik token → lemma + gloss → add to deck | GIF, devlog "kenapa screenshot bukan RAM" |
| B | "Emulator yang tidak pernah menyentuh internet" | Screenshot network adapter mati, fitur tetap jalan | Post r/Emulation + video "offline test" |
| C | "Belajar JP sambil main Emerald, terjemahannya bahasa Indonesia" | Screenshot overlay JP→ID pada dialog Emerald | Thread X + video CLi di YouTube |
| D | "Rust + llama.cpp + Tauri, semua lokal, 4B cukup" | Angka latency/CER dari `eval/`, ADR, source di GitHub | Blog teknis, HN "Show HN" |

## 4. Saluran & urutan gerak

Aturan: satu aset besar (video demo 60–90 detik) → potong jadi banyak postingan kecil. Jangan bikin 10 aset terpisah.

**Gelombang 0 — audience test (sebelum produk jadi, biaya ~0)**
- Tautan "waitlist" sesederhana halaman statis atau GitHub repo dengan badge "WIP". Tujuan tunggal: validasi apakah positioning "offline" atau "vocab/SRS" yang lebih dulu menarik. Ukur: klik dari masing-masing 3 post.
- Post teks + 1 gambar wireframe ke r/LearnJapanese dan r/Emulation, minta feedback fitur. Tidak menjual apa pun.

**Gelombang 1 — gebyar demo (setelah T3.3 hijau, target: video 1 menit Emerald JA → overlay ID)**
- Urutan posting: (1) GIF 15 detik ke r/Emulation + r/clonegaming; (2) devlog di blog pribadi/Substack + share ke Lobste.rs; (3) `Show HN: Open-Emulingo – learn Japanese by playing GBA games offline` (pilih Selasa–Kamis pagi WIB/US); (4) X/Twitter thread + Bluesky (ungi ke tag `#learnjapanese`, `#emulation`, `#localLLM`); (5) Discord: satu pesan di `#showcase`/`#projects`, jangan spam channel umum.
- Yang diukur per kanal: klik ke repo, star, "waitlist" (email), pertanyaan yang berulang (itu jadi FAQ).

**Gelombang 2 — kredibilitas (release v0.1 publik, installer Windows)**
- Rilis GitHub + `docs/DEMO.md`; kirim ke 3–5 newsletter/creator kecil (bukan besar): channel YouTube "belajar JP dari game", blogger emulation Indonesia, komunitas Refold/Discord kecil. Pitch yang dipakai: "kasih aku 20 menit, aku tunjukkan offline test".
- Distribusi sekunder: Scoop bucket / winget manifest (butuh maintainer, biaya rendah, sinyal legitimasi), Awesome-list PR (`awesome-emulation`, `awesome-language-learning`), Wikipedia/AlternativeTo page untuk pembanding (hanya setelah ada rilis).

**Gelombang 3 — komunitas & retensi**
- "Build in public" mingguan: satu devlog pendek tiap Jumat (apa yang jalan, angka eval, apa yang gagal). Ini mesin konten termurah untuk proyek solo.
- Kontribusi termudah untuk dijual sebagai partisipasi: `eval/` — user menambah screenshot+ground-truth game favorit mereka. Itu sekaligus marketing dan data produk.
- Second screen (F6) = fitur paling "shareable" (video TikTok/Shorts 15 detik HP+PC). Simpan untuk gelombang setelah v0.2.

## 5. Aset yang harus dibuat (minimal)

1. **Demo 60–90 detik**, tanpa musik, subtitle: cold-open "game berbunyi Jepang" → tekan T → overlay → klik kata → card SRS. Satu take, jangan editing berat.
2. **GIF 15 detik** (auto-translate trigger sendiri) untuk Reddit — Reddit menghukum video panjang di post pertama.
3. **Satu halaman landing** (satu file HTML, static, hosting GitHub Pages): klaim, GIF, syarat sistem (RX 9060 XT/16 GB sebagai patokan; NVIDIA Vulkan didukung `[verify]`), tombol Download + link source. Tidak ada analytics, sesuai semangat produk.
4. **Tabel pembanding** jujur: kita vs "VLM cloud + browser emulator" (Yomitan + browser, RetroArch + AI overlay) pada: offline, biaya, game coverage, kualitas OCR, privasi. Kalah di kualitas OCR = tulis apa adanya; kredibilitas lebih berharga.
5. **FAQ**: legal (ROM milik sendiri, BIOS replacement), apakah bisa HP (belum), VRAM minimum, apakah bisa bahasa lain (arsitektur iya, kamus id yang jadi nilai jual).

## 6. Anggaran & waktu (mode solo)

- **Uang:** mendekati nol. Domain ~$12/tahun `[verify]`, sisanya waktu. Tidak ada iklan berbayar sampai gelombang 1 menghasilkan sinyal organik.
- **Waktu:** 3–5 jam/minggu untuk marketing setelah ada demo; jangan lebih, itu mencuri jam dari T4/T5 yang membuat produk layak dijual.
- **Metrik tunggal yang dipantau:** jumlah orang yang benar-benar menjalankan app dan kembali seminggu kemudian. Star, upvote, dan pageview adalah vanity; satu baris di FAQ atau issue "berhasil jalan di GPU saya" lebih berharga.

## 7. Risiko & batas

| Risiko | Kenyataan | Tindakan |
|---|---|---|
| **Legal — ROM & trademark** | Kita tidak mendistribusi ROM/BIOS Nintendo. Nama tidak boleh memuat "Emulingo" (AGENTS.md §7). | Nama final harus lahir dari T6.7, dicek dulu: pencarian GitHub/npm/crates + USPTO/tanda-dagang negara tujuan. "Open-Emulingo" hanya internal. |
| **"Emulingo offline" sebagai framing** | Klaim "clone" mengundang DMCA dan perbandingan langsung yang merugikan (kita lebih lambat, lebih ribet). | Framing: "offline language-learning tool for GBA players". Sebut "inspired by browser-based tools" sekali, tanpa nama merek. |
| **Klaim "100% offline" vs kenyataan** | Model diunduh sekali via `scripts/setup-models.ps1` (network saat setup). | Wording tepat: "no network at runtime". Ini klaim yang bisa dibuktikan dan tetap kuat. |
| **OCR 4B kalah dari Gemini** | Realistis: model 4B salah baca kanji kecil. `00-master-plan.md` §7 sudah mengakui. | Jangan klaim akurasi. Klaim privasi/biaya/kemampuan offline, tunjukkan angka CER sendiri apa adanya. |
| **Over-promise F5/F6** | Belum ada kode. | Materi publik hanya menyebut fitur yang sudah bisa didemokan. Sisanya "roadmap". |
| **Hype tanpa produk** | Repo masih T0.0. | Gelombang 0 hanya untuk validasi; tidak ada link unduhan sampai ada build yang jalan pada mesin lain. |

## 8. Definition of done untuk plan ini

- Nama kerja diganti nama final, dicek bentrok.
- Demo T3.3 direkam, GIF dipotong, landing page satu file terbit.
- Tiga post gelombang 1 terjadwal (Reddit, HN, X) dengan naskah siap-tempel.
- Satu baris metrik dipilih dan dicatat mingguan.