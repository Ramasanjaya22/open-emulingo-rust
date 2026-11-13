# Copy — siap tempel per kanal

Status: draft, 2026-11-13. Setiap blok di file ini adalah naskah final yang bisa
di-copy-paste apa adanya. Ganti `{{LINK}}`, `{{HANDLE}}`, `{{DATE}}` sebelum kirim.

Dua placeholder yang harus diselesaikan manusia sebelum ada yang dikirim:

- `{{NAME}}` — nama produk publik. **Belum ada.** T6.7 belum dijalankan, dan `AGENTS.md`
  §7 + `docs/plan/00-master-plan.md` §7 melarang nama bermerek "Emulingo". Jangan pernah
  menaruh `open-emulingo` di materi publik; itu nama repo internal.
- `{{LINK}}` — belum ada URL. Tidak ada unduhan sampai G3.

## 0. Aturan copy (berlaku untuk semua blok)

Empat hal yang dilarang muncul di mana pun:

1. Nama merek "Emulingo", dalam bentuk apa pun.
2. Kata "clone" sebagai framing utama. "Inspired by browser-based tools" — sekali, tanpa nama.
3. Klaim akurasi ("akurat", "tepat", "seakurat Gemini"). Yang boleh: privasi, biaya, offline.
4. Janji tanggal atau fitur F5/F6 sebelum G5. Yang belum ada disebut "roadmap".

Wajib ada di setiap materi:

- Wording offline yang tepat: **"no network at runtime"**, bukan "100% offline". Alasannya
  ada di `scripts/setup-models.ps1` — model diunduh sekali saat setup, jadi "offline selamanya"
  adalah klaim yang bisa dibantah pembaca dalam satu kalimat.
- Syarat sistem: Windows 11, GPU Vulkan 8 GB+ (patokan RX 9060 XT 16 GB).
- Kalimat legal: "Kamu pakai ROM GBA milikmu sendiri. Aplikasi ini tidak menyertakan ROM, BIOS, atau game apa pun."

Empat hal yang tidak boleh dilakukan di kolom komentar:

- Menjawab "nanti rilis" untuk pertanyaan tanggal. Jawab: belum ada tanggal, ini progres.
- Membela diri saat dituduh clone. Jawab sekali dengan fakta arsitektur, lalu berhenti.
- Menyebut harga sebelum ada tier yang benar-benar dibuka.
- Membandingkan langsung ke produk berbayar per-token dengan angka yang belum diukur.

Nada: tenang, teknis, sedikit kering. Bukan hype. Target pembaca adalah orang yang sudah
tahu apa itu VLM dan sudah pernah bayar untuk auto-translate — mereka mencium marketing.

---

## 1. Positioning block (pakai ulang di mana saja)

Versi 25 kata, untuk bio dan footer:

> Belajar Jepang dari game GBA yang kamu sudah punya. Layar diterjemahkan, kata dijelaskan,
> kamus offline, tanpa network saat runtime, di GPU-mu sendiri.

Versi 12 kata, untuk judul post dan_subject line:

> Your games. Your GPU. No network at runtime.

Versi satu kalimat, untuk pembuka presentasi:

> Game Boy-mu mengajari bahasa Jepang, dan tidak ada satu paket pun yang keluar dari PC-mu.

---

## 2. Landing page

### 2.1 Hero

**H1:** Main dulu. Terjemahannya menyusul.

**Sub:** Belajar bahasa Jepang dari game GBA yang sudah kamu punya. Teks layar diterjemahkan
ke bahasa Indonesia atau Inggris, tiap kata bisa dijelaskan dan masuk ke jadwal ulang.
Satu model kecil, jalan di GPU-mu, tanpa network saat runtime.

**Tombol:** `Download (Windows)` — belum ada, `Coming soon` + `Lihat source` sampai G3.
Satu tombol saja di hero. Dua tombol membagi perhatian; satu tombol mengubah pikiran.

**Baris di bawah tombol:** Windows 11 · GPU Vulkan 8 GB+ · tidak butuh akun

### 2.2 Tiga klaim

**1. Layar diterjemahkan, bukan RAM dibaca.**
Kami tidak menyuntik kode ke game. Teks dibaca dari piksel layar, jadi satu build mengejar
semua GBA RPG/adventure — bukan cuma judul yang sudah ditambal orang lain.

**2. Tidak ada network saat runtime.**
Kabel LAN dicabut, semua fitur tetap jalan. Model diunduh sekali saat setup, lalu tidak ada
permintaan keluar lagi. Ini privasi, dan ini juga tagihan yang tidak pernah datang.

**3. GPU-mu yang kerja.**
Model 4B, pengenal suara, dan pengubah suara semuanya di VRAM lokal. Tidak ada langganan
per-token, tidak ada antrean, tidak ada layar yang dikirim ke server siapa pun.

### 2.3 Cara kerja (3 langkah)

1. **Buka ROM-mu.** File dialog biasa. Tidak ada folder khusus, tidak ada impor.
2. **Tekan T.** Layar dibekukan, dibaca, ditumpuk. Teks asli, furigana, dan terjemahan
   muncul berdampingan.
3. **Klik kata.** Definisi dari kamus offline, contoh kalimat asli dari game, tombol
   "tambah ke deck". Selesai.

### 2.4 Syarat sistem (jujur, tanpa pembungkus)

| | |
|---|---|
| OS | Windows 11 (Windows 10 tidak diuji) |
| GPU | Vulkan, 8 GB VRAM minimum; patokan RX 9060 XT 16 GB |
| RAM | 16 GB `[verify]` |
| Disk | ~4 GB untuk model + app |
| Setup | sekali, butuh internet; setelah itu tidak ada network saat runtime |
| Akun | tidak ada. Tidak ada email, tidak ada login. |

### 2.5 FAQ (naskah jawaban, bukan topik)

**"Apakah bisa di laptop?"**
Kalau laptopnya punya GPU Vulkan 8 GB ke atas, bisa. Kalau cuma iGPU, model 4B akan lambat
dan saya belum mengukurnya — jadi saya tidak akan bilang bisa.

**"Jadi ini emulator GBA?"**
Ya, tapi itu bukan jualannya. Emulator GBA sudah ada yang jauh lebih baik dan lebih cepat.
Ini alat belajar yang kebetulan menjalankan emulator supaya kamu tidak perlu dua aplikasi.

**"Apakah bisa bahasa selain Jepang?"**
Arsitekturnya bisa; yang belum ada adalah kamus dan data evaluasinya. Jepang dulu, Inggris
sekunder.

**"Kenapa gratis?"**
Karena saya tidak menanggung biaya inference — GPU-mu yang menanggung. Yang saya minta
hanya laporan kalau ada yang rusak.

**"Apakah ini legal?"**
Kamu pakai ROM milikmu sendiri. Aplikasi ini tidak menyertakan ROM, BIOS, atau game apa pun.
Emulator dan model yang dipakai semuanya open-source dengan lisensi masing-masing.

**"Kenapa tidak pakai Gemini saja?"**
Pakai saja, kalau kamu tidak masalah layar game-mu dikirim ke server dan tagihan yang
mengikuti. Bedanya: ini jalan tanpa internet setelah setup.

**"Apakah bisa di HP?"**
Belum. Arsitekturnya tidak menutup kemungkinan itu, tapi tidak ada rencana dekat.

### 2.6 Blok legal (wajib, jangan disembunyikan)

> Aplikasi ini tidak menyertakan ROM, BIOS, atau data game apa pun. Kamu harus memiliki
> salinan game yang kamu jalankan sendiri. Nama "GBA" dan "Game Boy Advance" adalah merek
> Nintendo; kami tidak berafiliasi. Atribusi lisensi lengkap ada di installer.

### 2.7 Blok sponsor (satu paragraf, di footer)

> Proyek ini dibangun solo dan tidak menagih apa pun. Kalau kamu ingin membantu, ada dua
> cara yang benar-benar berguna: laporkan bug dengan detail mesinmu, atau tambahkan
> screenshot + teks asli dari game favoritmu ke set evaluasi. Keduanya membuat produk lebih
> baik untuk semua orang.

---

## 3. Show HN

**Judul:** Show HN: Belajar Jepang dari game GBA, offline di GPU sendiri

**Body:**

> Saya sedang membangun alat belajar bahasa Jepang yang membaca teks dari layar game GBA
> secara langsung dari piksel, lalu menerjemahkannya ke bahasa Indonesia atau Inggris. Semua
> jalan di GPU lokal; tidak ada permintaan network saat runtime.
>
> Kenapa dari piksel, bukan dari RAM: kalau teks dibaca dari piksel layar, satu build bisa
> mengejar semua game — tidak perlu tambalan per judul, tidak perlu tabel alamat. Harganya
> adalah model 4B harus membaca font piksel Jepang yang kecil, dan saya belum punya angka
> untuk itu. Itu justru yang sedang saya ukur sekarang.
>
> Yang sudah jalan: emulator, render 240×160 integer-scaled 4× pada 59,73 fps, dan
> translator manual dengan furigana per baris. Yang belum: auto-translate dan kamus.
>
> Stack: Rust, Tauri 2, llama.cpp Vulkan, Qwen 4B, Whisper small, Kokoro. Semuanya lokal.
>
> Saya mencari dua hal: (1) orang yang mau menguji di GPU non-AMD dan melaporkan apa yang
> terjadi, (2) koreksi keras untuk pendekatan piksel-saja ini — apakah menurut kalian ada
> yang saya lewatkan yang akan membuatnya runtuh di game lain?
>
> Repo: {{LINK}}

**Catatan operasional:** Selasa–Kamis, pagi waktu US. Jangan posting di Jumat sore.
Siapkan diri untuk satu pertanyaan yang pasti muncul: "kenapa tidak pakai OCR biasa?"
Jawaban yang sudah disiapkan di §7.

---

## 4. Reddit — r/Emulation

**Judul:** I'm building a GBA emulator that learns Japanese with you — reading text from
pixels instead of RAM, all local

**Body:**

> Sudah lama ini mengganggu saya: kalau saya mau belajar Jepang sambil main GBA, saya
> butuh dua hal — emulator, dan alat terjemahan yang mengirim screenshot saya ke server
> orang lain. Saya tidak mau yang kedua.
>
> Jadi saya buat yang jalan di GPU sendiri. Emulator Rust, frame 240×160 dirender
> integer-scaled 4×, teks dibaca dari piksel layar oleh model 4B lokal. Furigana per baris,
> klik kata dapat definisi, terjemahan ke Indonesia atau Inggris.
>
> Yang jujur: ini lebih lambat dari emulator yang kamu pakai sekarang, dan model 4B akan
> salah baca kanji kecil. Saya tidak akan mengklaim akurasi. Yang saya klaim cuma satu:
> cabut kabel LAN, semua fitur tetap jalan.
>
> Belum ada rilis. Saya di sini untuk satu pertanyaan: kalau kamu pakai emulator dengan
> VLM, apa yang paling sering bikin kamu berhenti — akurasi, latency, atau setup?
>
> Repo: {{LINK}}

**Catatan operasional:** ajukan satu pertanyaan di akhir, dan jawab setiap balasan dalam 90
menit pertama. r/Emulation menghukum drive-by post lebih keras daripada konten yang sedang
dibangun; kalau akunnya baru, hangatkan dulu dengan komentar non-promosi seminggu
sebelumnya.

---

## 5. Reddit — r/clonegaming

**Judul:** An offline GBA learning setup — furigana, tap-to-define, no cloud

**Body:**

> Postingan ini bukan "download emulator terbaik". Ini setup yang saya pakai untuk belajar
> Jepang lewat Emerald: layar dibaca model lokal, tiap baris dapat furigana, tiap kata bisa
> dijelaskan dan dimasukkan ke deck ulang.
>
> Yang bikin saya bangun sendiri: tidak ada satu pun alat belajar yang mau jalan tanpa
> mengirim layar game ke server. Kalau kamu alergi cloud AI, ini jawabannya.
>
> Beta tertutup, belum ada build publik. Yang saya butuh sekarang: 5–10 orang dengan GPU
> AMD/Intel/NVIDIA yang mau uji dan mengirim log kalau gagal.
>
> {{LINK}}

---

## 6. Reddit — r/LearnJapanese (dan r/Anki)

**Judul:** Tool: learn Japanese from GBA games you already own, entirely on your own GPU

**Body:**

> Konteks singkat: saya sudah pakai Yomitan + Anki untuk immersion, dan yang selalu hilang
> adalah GBA — layar 240×160 dengan font Jepang kecil, tidak ada teks yang bisa di-copy,
> tidak ada integrasi apa pun.
>
> Yang saya buat: emulator yang membaca teks dari piksel layar pakai model 4B lokal, lalu
> menampilkan teks asli, furigana, dan terjemahan. Klik kata → definisi dari kamus offline
> (JMdict) → "tambah ke deck" → FSRS. Semua di GPU sendiri, tanpa network saat runtime.
>
> Kenapa ini penting untuk kalian: alat yang ada mengirim screenshot ke Gemini/OpenAI, jadi
> tidak bisa dipakai offline dan tagihannya mengikuti pemakaian. Ini tidak.
>
> Beta tertutup. Yang saya cari: 10 orang yang mau uji 20 menit dan melaporkan latency plus
> apakah auto-translate tersendat atau tidak. Saya tidak mencari pujian; saya mencari
> kegagalan.
>
> {{LINK}}

**Catatan operasional:** aturan r/LearnJapanese melarang promosi tanpa nilai; post ini
membawa nilai (setup nyata untuk gap yang nyata) tapi tetap siap dihapus. Bawa tangkapan
layar, bukan janji. Kalau ditolak, pindah ke r/Anki dengan sudut "FSRS + game immersion",
bukan "emulator".

---

## 7. Devlog (blog / Substack) — gelombang 1

**Judul:** Minggu 1: model 4B membaca font piksel Jepang, dan angkanya tidak enak

**Body:**

> Pembukaan yang tidak menyenangkan: rencana saya adalah satu model 4B untuk semua hal —
> OCR, terjemahan, penjelasan tata bahasa. Setelah seminggu, saya punya angka pertama, dan
> angkanya cukup jelek untuk kanji kecil.
>
> Yang saya ukur: 40 screenshot Emerald versi Jepang, dengan teks asli yang saya verifikasi
> manual (kana dan kanji harus tepat, tidak boleh ada tebakan). Dua varian input: mentah
> 240×160, dan di-upscale nearest 4×. Metriknya CER — berapa persen karakter yang salah baca.
>
> Yang saya temukan: upscale 4× membantu, tapi tidak menyembuhkan. Font piksel Emerald
> didesain untuk mata manusia pada jarak 30 cm, bukan untuk encoder VLM. Beberapa kanji
> dengan garis padat hilang sepenuhnya, dan yang hilang justru yang paling sering muncul
> di dialog.
>
> Keputusan yang saya ambil: saya tidak akan mengklaim akurasi di materi pemasaran. Saya
> akan menerbitkan angka ini apa adanya, dan membangun jalur koreksi manual — user bisa
> memperbaiki hasil baca, dan koreksi itu masuk ke set evaluasi.
>
> Berikutnya: latency pada RX 9060 XT, dan apakah model 9B pantas dipertimbangkan.
>
> Kalau kamu punya game GBA lain dan mau menambahkan screenshot ke set evaluasi, itu
> bantuan paling berguna yang bisa kamu berikan sekarang.
>
> {{LINK}}

**Catatan operasional:** satu devlog per Jumat, 3–5 paragraf, satu angka konkret. Ini mesin
konten termurah untuk proyek solo: tidak perlu produksi, tidak perlu skrip.

---

## 8. X / Twitter thread (9 post)

**1/** Saya sedang membuat alat belajar Jepang yang membaca teks dari layar game GBA
langsung dari piksel, lalu menerjemahkannya di GPU saya sendiri. Tanpa network saat
runtime. Belum ada rilis — ini progres, bukan peluncuran. 🧵

**2/** Masalahnya: semua alat auto-translate untuk game mengirim screenshot ke server.
Gemini, OpenAI, apa pun. Berarti butuh internet, dan tagihan mengikuti pemakaian.

**3/** Saya tidak mau itu. Jadi: model 4B lokal, jalan di VRAM. Cabut kabel LAN, semuanya
tetap jalan.

**4/** Kenapa baca piksel, bukan RAM: kalau teks diambil dari piksel layar, satu build bisa
mengejar semua GBA RPG/adventure. Tidak ada tambalan per judul, tidak ada tabel alamat.

**5/** Onganinya jujur: model 4B harus membaca font piksel Jepang yang kecil, dan angka
pertama saya tidak enak. Saya ukur CER pada 40 screenshot Emerald Jepang dengan teks asli
yang saya verifikasi manual.

**6/** Upscale nearest 4× membantu, tidak menyembuhkan. Beberapa kanji padat hilang
sepenuhnya. Jadi: tidak ada klaim akurasi di materi pemasaran. Angkanya saya terbitkan apa
adanya.

**7/** Yang sudah jalan: emulator Rust, 240×160 di-upscale 4× ke jendela, 59,73 fps, dan
translator manual dengan furigana per baris. Klik kata → JMdict → masuk deck FSRS.

**8/** Yang belum: auto-translate, kamus, companion suara. Semuanya di plan, semuanya di
belakang gerbang yang harus lewat dulu.

**9/** Stack: Rust, Tauri 2, llama.cpp Vulkan, Qwen 4B, Whisper small, Kokoro. Semua
lokal. Beta tertutup — yang saya butuh: orang dengan GPU non-AMD untuk uji dan laporan
kegagalan. {{LINK}}

**Catatan operasional:** post 1 dan 5 adalah kandidat unggulan. Jangan bikin thread 9 post
untuk rilis pertama — itu bikin orang berhenti di post 3. Satu gambar per 2 post.

---

## 9. Satu pesan Discord (untuk komunitas JP-learning)

Tempel di `#showcase` atau `#projects`, bukan channel umum. Jangan spam.

> Hai — saya sedang membangun alat belajar Jepang yang membaca teks dari layar game GBA
> (dari piksel, bukan RAM) dan menerjemahkannya ke Indonesia/Inggris, semuanya di GPU lokal
> tanpa network saat runtime. Stack: Rust + Tauri + llama.cpp Vulkan + Qwen 4B.
>
> Saya bukan di sini untuk promosi. Saya mencari 5–10 orang yang mau uji 20 menit dan
> mengirim log kalau gagal — terutama yang punya GPU NVIDIA atau Intel, karena saya cuma
> bisa menguji AMD. Kalau tertarik, DM saya.
>
> Belum ada build publik, jadi ini murni beta uji.

**Yang tidak boleh:** menyebut tier sponsor, menyebut harga, atau membalas DM dengan
tautan donasi. Beta uji dulu, uang nanti.

---

## 10. Balasan untuk keberatan yang sudah bisa diduga

**"Ini clone dari X, kan?"**
> Saya tidak menyalin apa pun dari sana. Bedanya arsitektur: mereka membaca teks dengan VLM
> cloud dari screenshot; saya menjalankan model 4B di GPU sendiri dan tidak mengirim apa pun
> keluar. Itu keputusan teknis, bukan gimmick pemasaran.

**"Kenapa tidak pakai OCR biasa seperti PP-OCR?"**
> Saya mengujinya (rencana T0.4). OCR khusus lebih cepat dan lebih murah, tapi butuh model
> per bahasa dan tidak bisa menjelaskan tata bahasa atau menjawab pertanyaan tentang baris
> itu. Satu model yang bisa membaca, menerjemahkan, dan mengajar lebih sederhana untuk
> dipelihara solo — meski lebih lambat. Saya akan terbitkan angka perbandingannya.

**"4B tidak akan pernah cukup bagus."**
> Mungkin benar. Itu sebabnya set evaluasi dan angka CER saya terbitkan sebelum ada klaim.
> Kalau 4B gagal di eval, saya aktifkan 9B (16 GB VRAM cukup untuk keduanya) atau saya
> hentikan proyek ini dan bilang begitu.

**"Kenapa tidak ada versi Mac/Linux?"**
> Karena saya cuma punya satu mesin Windows dan tidak bisa menguji yang lain. Menambah
> target tanpa alat uji = janji yang tidak bisa saya tepati.

**"Apakah ini legal? Emulator itu..."**
> Kamu pakai ROM milikmu sendiri. Aplikasi ini tidak menyertakan ROM, BIOS, atau data game.
> Emulator yang saya pakai open-source, modelnya open-source, dan atribusi lisensi lengkap
> ada di installer.

---

## 11. Naskah video demo 60–90 detik (shot list, bukan naskah pidato)

Tanpa musik, subtitle saja. Satu take, tanpa editing berat. Tujuan tunggal: orang percaya
ini benar-benar jalan di luar mesin pembuatnya.

| # | Durasi | Yang terlihat | Teks di layar |
|---|---|---|---|
| 1 | 0–8 s | Emerald JA jalan normal, dialog mengalir, tidak ada overlay | "Game ini berbunyi Jepang." |
| 2 | 8–14 s | Tangan menekan **T**. Layar membeku, muncul overlay | "Tekan T." |
| 3 | 14–28 s | Overlay: teks asli, furigana, terjemahan Indonesia. Zoom ke satu baris | "Dibaca dari piksel layar, bukan dari RAM." |
| 4 | 28–40 s | Klik kata 伝説 → kartu definisi → tombol tambah ke deck | "Klik kata. Masuk deck." |
| 5 | 40–50 s | Kabel LAN dicabut dari laptop. Fitur tetap jalan | "Kabel LAN dicabut. Tetap jalan." |
| 6 | 50–65 s | Layar review FSRS: kartu, crop screenshot game, rating Again/Hard/Good/Easy | "Ulang dengan FSRS." |
| 7 | 65–80 s | Kartu companion: Kana menjawab, koreksi partikel muncul | "Roadmap — bukan janji." |
| 8 | 80–90 s | Kartu akhir: nama, syarat sistem, tautan source | "{{NAME}} · Windows 11 · GPU Vulkan 8 GB+" |

**Yang wajib ada di frame 5:** network adapter yang benar-benar mati. Tanpa itu, klaim
"offline" cuma tulisan. Ini shot yang menyelamatkan seluruh video.

**Yang tidak boleh ada:** tangkapan layar yang menampilkan teks bug, placeholder, atau
fitur yang belum ada. Satu frame "coming soon" menghancurkan kepercayaan pada 8 detik
sisanya.

---

## 12. Naskah presentasi workshop 90 menit (untuk segmen penjualan #2)

Struktur yang dijanjikan di sales plan §3, disusun supaya bisa dijalankan solo.

| Menit | Bagian | Isi |
|---|---|---|
| 0–10 | Setup bersama | Peserta install + `setup-models.ps1` + `run-llm.ps1`. Wajib lulus sebelum lanjut; yang gagal dapat bantuan 1-on-1. |
| 10–25 | Kenapa piksel, bukan RAM | Demo perbandingan: teks diambil dari layar vs tabel alamat. Tunjukkan kenapa yang kedua tidak portabel. |
| 25–45 | Membaca layar | Prompt, `json_schema`, kenapa output terstruktur penting, dan apa yang rusak pada kanji kecil. Angka CER dibuka di sini, apa adanya. |
| 45–60 | Latency | Ukur sendiri di mesin peserta. Ini bagian yang paling diingat: mereka melihat angka di hardware mereka, bukan di slide. |
| 60–75 | Deck ulang | Furigana → JMdict → FSRS. Peserta menambahkan 5 kartu dari game mereka sendiri. |
| 75–85 | Uji offline | Cabut network adapter. Semua fitur harus tetap jalan. |
| 85–90 | Pertanyaan + apa yang belum ada | Sebut roadmap tanpa tanggal. Jangan janji. |

**Yang dijanjikan ke pembeli:** 90 menit, maksimal 12 peserta, GPU Vulkan 8 GB+ di setiap
mesin. Yang tidak dijanjikan: tanggal rilis, fitur roadmap, atau dukungan setelah sesi.

**Harga:** $200/sesi `[verify: belum diuji ke pasar]`. Sesi pertama gratis untuk satu grup,
sebagai studi kasus dan sumber testimoni.

---

## 13. Dua naskah pendek untuk segmen kontributor

**Untuk kontributor packaging (Scoop/winget):**

> Butuh bantuan: manifest Scoop/winget untuk rilis Windows. Scope-nya sempit — satu file
> `.ps1` manifest, hash, dan uji install/uninstall. Kalau kamu sudah pernah bikin bucket,
> ini 2 jam kerja. Bayaran per task, terdefinisi di depan.

**Untuk kontributor adapter emulator (mGBA FFI):**

> Butuh bantuan: adapter `Emulator` kedua di belakang `rustboyadvance-ng`, supaya game yang
> gagal boot punya jalur cadangan. Trait-nya sudah ada, jadi yang dibutuhkan cuma
> implementasi + test boot. Bayaran per task, scope terkunci.

Keduanya menempel pada task yang sudah ada di plan (Fase 6, packaging; Fase 0.6, fallback
mGBA), bukan pekerjaan karangan.