# Sales Plan — open-emulingo-rust (nama kerja)

Status: draft, 2026-11-13. Menyusun `docs/marketing/plan.md` (T6.7) dan hanya menjual apa
yang sudah dibuktikan repo. Bukan eksekusi; angka yang belum diukur ditandai `[verify]`.
Aturan yang mengikat dokumen ini: `AGENTS.md` (nama tidak boleh memuat "Emulingo", tanpa
Python/React, offline saat runtime) dan `docs/plan/00-master-plan.md` §7.

## 1. Realita yang boleh dijual hari ini

Diverifikasi dari repo, 2026-11-13:

| Fakta | Bukti di repo |
|---|---|
| Belum ada produk: `crates/` dan `app/` tidak ada | `glob crates/*/Cargo.toml` → kosong |
| Bootstrap baru selesai | `git log` → `a8bceae T0.0: bootstrap` (2 commit) |
| Model stack sudah dipatok (llama.cpp b12344 Vulkan, Qwen3.5-4B Q4_K_M, mmproj F16) + SHA-256 | `models/README.md` |
| ROM, BIOS, save, dan model tidak pernah masuk repo | `.gitignore` |
| Fitur F1–F14 yang dijanjikan belum ada kodenya | `docs/plan/00-master-plan.md` §1 |

**Konsekuensi komersial:** belum ada yang bisa dijual sebagai produk. Yang bisa dijual
hari ini adalah akses awal dan bukti eksekusi, bukan lisensi. Materi publik hanya menyebut
F1–F4 (emulator, manual translate, auto-translate, history); sisanya "roadmap" — disiplin
yang sudah ditulis di plan marketing §0.

Nama produk publik belum ada (T6.7). Sampai itu selesai, semua aktivitas memakai
"open-emulingo" sebagai nama kerja internal saja: tidak ada domain bermerek, toko, atau akun
publik yang memakainya.

## 2. Positioning komersial

Satu kalimat, sama dengan plan marketing, dipakai apa adanya di semua kanal:

> Belajar bahasa Jepang dari game GBA yang kamu sudah punya — terjemahan layar, kamus, dan
> partner bicara, jalan tanpa network saat runtime di PC Windows-mu.

Tiga klaim yang boleh diulang, beserta batasnya:

1. **"No network at runtime."** Bukan "offline selamanya": model diunduh sekali oleh
   `scripts/setup-models.ps1` (butuh internet saat setup). Klaim yang bisa dibuktikan =
   runtime menolak host non-loopback.
2. **"Satu build untuk semua game."** Deteksi teks dari piksel layar, bukan baca RAM. Ini
   klaim arsitektur yang sudah diputuskan, bukan klaim akurasi.
3. **"GPU-mu yang kerja."** 4B + Whisper + Kokoro lokal, patokan RX 9060 XT 16 GB. NVIDIA
   Vulkan didukung arsitektur `[verify: belum diuji]`.

Yang tidak boleh diklaim: akurasi OCR, "lebih baik dari Gemini", dan nama "Emulingo". OCR 4B
pada kanji kecil hampir pasti kalah; master plan §7 sudah mengakui ini lebih dulu. Menjual
privasi, biaya, dan offline itu jujur; menjual akurasi berujung retur.

## 3. Siapa yang bayar, dan apa yang mereka beli

Produk ini gratis, offline, dan tidak menagih per-token. Jadi "penjualan" bukan langganan;
ada tiga bentuk nilai yang nyata dan bisa ditagih.

| # | Pembeli | Yang mereka beli | Bentuk transaksi | Kesiapan |
|---|---|---|---|---|
| 1 | Early supporter komunitas JP-learning (Refold, r/Anki, Discord immersion) | Akses awal + nama di daftar pendukung + suara fitur | GitHub Sponsors / Ko-fi, tier tunggal | Bisa buka sekarang: tidak menjanjikan fitur, hanya akses |
| 2 | Kelas/komunitas kecil (kelas JLPT, Discord kampus, grup belajar JP) | Workshop 90 menit "belajar JP dari game", termasuk setup di mesin peserta | Paket per sesi `[verify: belum ada calon]` | Setelah G4 (v0.1 installer) — sebelum itu tidak bisa didemokan di mesin lain |
| 3 | Kontributor bayaran (opsional) | Kontrak kecil: packaging Scoop/winget, adapter emulator (mGBA FFI), kamus ja→id | Per task terdefinisi | Setelah G5 (v0.2) |

Ketiganya sengaja kecil dan tidak saling mengunci: proyek ini solo, dan target ekonominya
menutup biaya domain/infra plus membeli waktu, bukan membangun mesin revenue.

## 4. Harga (usulan, belum disetujui user)

| Item | Angka usulan | Alasan |
|---|---|---|
| Sponsor "Supporter" | $5/bln atau $40/tahun | Di bawah langganan tool sejenis; imbalannya akses, bukan lisensi |
| Sponsor "Patron" | $15/bln, maksimal 25 slot | Batas slot menjaga janji "bantu setup" tetap ditepati solo |
| Workshop komunitas | $200/sesi, 90 menit, maks 12 peserta | Harga waktu, bukan harga produk |
| Kontrak kontribusi | $30–60 per task terdefinisi | Harga scope kecil dan jelas |
| Lisensi komersial produk | tidak dijual | Lisensi model/dict campur + atribusi wajib membuat ini mahal dijaga; gratis + sponsor lebih murah |

Angka di tabel ini belum disetujui user dan belum diuji ke pasar. Harga Rp0 untuk produk inti
layak dipertahankan: dengan harga nol, klaim "offline/tanpa biaya" tidak bisa dituduh
pemasaran-tipu.

## 5. Urutan gerak (mengikuti critical path, bukan tanggal)

Aturan: jangan menjual apa pun sebelum gerbang buktinya lewat. Satu gerbang = satu aset.

| Gerbang | Bukti yang harus ada | Aktivitas penjualan |
|---|---|---|
| G0 — sekarang | Repo + plan publik | Buka GitHub Sponsors tier Supporter saja. Tidak ada link unduhan. Satu post "sedang dibangun" (teks + wireframe) ke r/LearnJapanese dan r/Emulation. Ukur klik, bukan pujian. |
| G1 — setelah T0.3 hijau (angka OCR/latency 4B) | `eval/report-ocr.md` berisi CER + latency | Devlog pertama yang memuat angka. Ini aset kredibilitas termurah yang ada. Sebut sponsor satu baris di footer. |
| G2 — setelah T0.6 hijau (Emerald boot 60 fps, replacement BIOS) | Log fps, save in-game jalan | GIF 15 detik emulator jalan dengan network adapter mati, ke r/Emulation + r/retrogaming. Buka tier Patron (maks 25). |
| G3 — setelah T3.3 hijau (auto-translate di Emerald JA) | Demo 60–90 detik: teks Jepang → overlay Indonesia | Show HN + X/Bluesky thread, landing page satu file (GitHub Pages). Ini momen jualan sesungguhnya; sebelumnya pemanasan. |
| G4 — setelah v0.1 (installer Windows) | `cargo tauri build` dipasang di mesin lain | Workshop komunitas pertama (gratis untuk satu grup sebagai studi kasus). Distribusi sekunder: Scoop bucket/winget manifest, PR awesome-list. |
| G5 — setelah v0.2 (SRS + companion) | F4/F5/F9/F10 bisa didemokan | Baru di sini F5/F6 disebut sebagai produk, bukan roadmap. Kontrak kontribusi dibuka. |

Yang tidak boleh terjadi: menyebut F5/F6 di materi publik sebelum G5, dan menjanjikan apa pun
sebelum ada build yang jalan di mesin selain mesin pengembang.

## 6. Materi penjualan minimum

1. Landing page satu file (HTML statis, GitHub Pages): satu klaim, GIF, syarat sistem,
   tombol Source + Sponsor. Tanpa analytics.
2. Tabel pembanding jujur: kita vs "Yomitan + browser" vs "RetroArch + overlay AI cloud",
   pada offline, biaya, cakupan game, kualitas OCR, privasi. Kolom OCR kita kalah — tulis.
3. FAQ: legal (ROM milik sendiri, BIOS replacement), VRAM minimum, "kenapa gratis",
   "bisa di HP?" (belum), "bahasa lain?" (arsitektur iya, kamus id jadi nilai jual).
4. Naskah siap-tempel untuk 5 kanal: r/LearnJapanese, r/Emulation, Show HN, X thread, satu
   pesan Discord. Satu naskah per kanal, bukan satu naskah disebar.

## 7. Metrik, biaya, waktu

- Metrik tunggal yang dipantau: jumlah orang yang benar-benar menjalankan app dan kembali 7
  hari kemudian. Star, upvote, pageview = vanity.
- Metrik sponsor: jumlah Patron aktif, dan apakah janji "bantu setup" masih ditepati dalam
  48 jam. Kalau tidak, naikkan harga atau tutup tier.
- Biaya: domain ~$12/tahun `[verify]`; sisanya waktu. Tidak ada iklan berbayar sebelum G3
  menghasilkan sinyal organik.
- Waktu: maksimal 3–5 jam/minggu untuk penjualan setelah G1. Lebih dari itu mencuri jam dari
  T4/T5 yang membuat produk layak dibeli.

## 8. Risiko komersial

| Risiko | Kenyataan | Tindakan |
|---|---|---|
| Menjual sebelum ada produk | Repo masih T0.0 | Tier Supporter = akses, bukan fitur. Tanpa janji tanggal. |
| Nama "open-emulingo" dipakai di luar | AGENTS.md §7 + master plan §7 melarang merek | Nama publik lahir dari T6.7, dicek GitHub/npm/crates + tanda-dagang. Sampai itu: nama kerja. |
| Sponsor menuntut fitur | Solo dev, throughput terbatas | Patron dibatasi 25 slot; yang dijual waktu setup, bukan roadmap. |
| Lisensi model/dict | JMdict CC-BY-SA, Wiktionary CC-BY-SA, Qwen, Whisper, Kokoro, emulator | Atribusi + daftar lisensi di installer (task T6.6). Jangan jual lisensi komersial sebelum ini beres. |
| Klaim offline vs unduhan setup | Setup butuh internet | Wording tepat: "no network at runtime" (§2). |
| Workshop gagal di mesin peserta | Hardware bervariasi | Batasi ke GPU Vulkan 8 GB+; verifikasi dulu, jangan janjikan semua laptop. |

## 9. Definition of done untuk plan ini

- User memutuskan: harga sponsor (ya/tidak, berapa), dan apakah workshop/kontrak dikejar.
- Nama publik final dari T6.7, sudah dicek bentrok.
- Satu baris metrik dipilih dan dicatat mingguan.
- Tiga naskah kanal (r/LearnJapanese, r/Emulation, Show HN) ditulis sebelum G3.