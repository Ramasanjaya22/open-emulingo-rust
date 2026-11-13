# Aset visual & prompt image-gen — OpenKotoba (nama kerja, T6.7)

Status: 2026-11-13. `generate_image` di sesi ini **tidak bisa dipakai** — endpoint menolak model bawaan dan tidak punya kredensial (`No credentials for image provider: openai` / `codex`). Karena itu:

- Logo, wordmark, dan mockup app **sudah dibuat langsung** sebagai SVG + HTML, lalu dirender ke PNG (lihat daftar file di bawah).
- Prompt siap-tempel untuk ChatGPT image-gen (gpt-image-2.x) ada di §3 — pakai itu kalau mau versi ilustratif/ray-traced yang tidak bisa dihasilkan SVG.

## 1. Keputusan desain

Sumber: skill `clean-apple-design` (wajib untuk semua UI di repo ini) + `00-master-plan.md` §5 (light mode, no gradient).

| Token | Nilai |
|---|---|
| Canvas | `#FFFFFF` |
| Panel / stage | `#F5F5F7` |
| Garis | `#E4E4E7` |
| Teks utama | `#1D1D1F` |
| Teks sekunder | `#6E6E73` |
| Aksen tunggal | `#D9542F` (vermilion) |

Alasan aksen vermilion: satu aksen hangat, cukup kontras di putih (rasio ≈ 3.6:1 — **cukup untuk elemen grafis dan teks besar, tidak untuk teks isi kecil**; jangan pakai vermilion untuk body text), dan merujuk hanko/seal Jepang tanpa jadi klise sakura-merah.

Mark: **speech bubble yang bagian bawahnya berubah menjadi grid piksel persegi** — dialog box game portabel yang mengendap jadi bahasa. Sengaja bisa dibaca di 16 px (monokrom, tanpa detail halus).

## 2. File yang sudah jadi

Semua di `docs/marketing/visuals/`:

| File | Isi |
|---|---|
| `logo-mark.svg` | Mark 512×512, viewBox bersih, `currentColor`-free, aman untuk favicon |
| `logo-lockup.svg` | Lockup 1200×300: mark + wordmark "OpenKotoba" + tagline |
| `logo-mark@1024.png` | Render 1024×1024 untuk app icon / store listing |
| `logo-lockup@2x.png` | Render 2400×600 (retina) untuk README dan landing |
| `app-hifi.html` | Mockup 3 layar (play+translate, review, companion) dalam satu file, 1600×1000 per layar |
| `shot-play@2x.png` | Layar main + panel Translation |
| `shot-review@2x.png` | Layar Review (flashcard + FSRS rating) |
| `shot-companion@2x.png` | Panel companion + latihan bicara |

Render ulang kalau `app-hifi.html` diubah:

```js
const w = new Bun.WebView({ width: 1602, height: 1000 });
await w.navigate("file:///F:/@workspace/open-emulingo-rust/docs/marketing/visuals/app-hifi.html#play");
await Bun.write(".../shot-play@2x.png", await w.screenshot());
```

Catatan: mockup ini **fiktif**. Game yang ditampilkan pixel art generik, bukan Emerald asli — supaya tidak meniru aset Nintendo (AGENTS.md §1 dan risiko legal di `docs/marketing/plan.md` §7). Pixel art-nya sengaja diagramatik, karena screenshot asli baru ada setelah T1.4 + T2.4 hijau.

## 3. Prompt untuk ChatGPT image-gen

Model yang dipakai saat menulis: `gpt-image-2.6`. Tempel utuh, jangan dipotong — bagian "style block" yang menjaga konsistensi antar aset.

### Style block (tempel di depan setiap prompt)

```
Style: light mode only. Canvas #FFFFFF, panel #F5F5F7, hairline border #E4E4E7, primary text #1D1D1F, secondary text #6E6E73, one restrained accent vermilion #D9542F. Typography: SF Pro Display-like geometric system sans, generous letter-spacing on small uppercase labels. No gradients, no neon, no glassmorphism, no heavy shadows, moderate consistent corner radii, generous whitespace. Flat front-facing orthographic view, no perspective, no device frame, no browser chrome, no people, no stock photography, no 3D bevel. Crisp pixel-perfect rendering, looks like a real shipping desktop app on Windows 11.
```

### 3.1 Logo — mark

```
[STYLE BLOCK]

TASK: app icon, square 1024x1024, canvas fills the frame edge to edge.
The mark: a clean geometric speech bubble whose lower half resolves into a 5x4 grid of chunky square pixels — a retro handheld dialog box turning into language. Flat vermilion #D9542F shape, two of the pixels filled charcoal #1D1D1F at 22% opacity for depth. Thick uniform stroke weight, symmetric, no outline box, no letters, no gradient, no bevel, no 3D, no mockup, no drop shadow, no text.
```

### 3.2 Logo — wordmark lockup

```
[STYLE BLOCK]

TASK: horizontal logo lockup on a plain white field, 1600x600, generous margins, optically centered.
Left: a 96px square badge containing the mark described above (speech bubble resolving into square pixels, vermilion #D9542F, two charcoal pixels).
Right of the badge: the wordmark "OpenKotoba" in a semibold geometric sans, charcoal #1D1D1F, tight tracking, lowercase k, large and confident. Directly beneath it a small letterspaced uppercase tagline "LEARN JAPANESE BY PLAYING" in #6E6E73 at 11px.
Nothing else on the field. No tagline box, no decoration, no mockup scene.
```

### 3.3 Screenshot hi-fi — play + translation

```
[STYLE BLOCK]

TASK: desktop application UI mockup, 1600x1000, main gameplay screen with the translation overlay open.
Layout: slim 44px title bar, small vermilion logo mark at the left, centered document title "Emerald — BPEJ", three tiny neutral window dots at the right.
Below: two columns.
LEFT (flex): light gray stage #F5F5F7 with 36px padding, holding a pixel-art handheld game screen at 3:2 aspect ratio, crisp low-resolution pixel art of a GENERIC fictional Japanese fantasy RPG — a small pixel character on a tiled path, two trees, and a plain white dialog box along the bottom containing two lines of small Japanese text and a blinking downward triangle. The pixel art must not resemble any existing commercial game. Beneath the screen, a small caption row: "240 × 160 · integer scaled 4×" on the left and "59.73 fps" on the right, in #6E6E73 at 12px.
RIGHT (380px, white, hairline left border): a header row with the title "Translation" and an ID / EN segmented toggle whose ID half is filled vermilion. Two stacked translation rows separated by hairline borders; each row shows a line of Japanese in charcoal at 17px, a smaller vermilion hiragana reading above it at 12.5px, and an Indonesian translation in #6E6E73 at 13.5px. Then a small uppercase label "WORDS IN THIS LINE" in #6E6E73, followed by three pill-shaped word chips with hairline borders, each showing a Japanese term, a small reading and gloss, and a small vermilion circular plus icon.
Bottom strip of the right panel, above a hairline border: a tiny closed padlock glyph and the text "Offline — all processing on this device" in #6E6E73 at 12px.
No scrollbars, no window controls beyond the three dots, no desktop wallpaper.
```

### 3.4 Screenshot hi-fi — review / SRS

```
[STYLE BLOCK]

TASK: desktop application UI mockup, 1600x1000, vocabulary review screen.
Layout: same slim 44px title bar, vermilion mark at the left, centered title "Review".
LEFT SIDEBAR (240px, #F5F5F7, hairline right border): five nav rows with small monoline stroke icons — Play, Translate, Review, Library, Settings. Review is current: white background, vermilion icon, and a 2px vermilion bar inset on its left edge. Pinned to the sidebar bottom, above a hairline border: a 52px circular progress ring filled 68% in vermilion with the number "128" inside it, the label "cards due" in #6E6E73, and below that one small row reading "XP 1,240   streak 12   new 8".
MAIN: one centered white flashcard, 720x460, hairline border, 14px radius, one very soft ambient shadow. Top-left small uppercase kicker "JAPANESE TO INDONESIAN" in #6E6E73. Centered: one large Japanese word in charcoal at 56px, with its hiragana reading directly above in vermilion at 20px. Beneath the word, a 16:9 inset crop of the original game dialog box — pixel art with a white box, black text bars, and a small vermilion triangle — hairline border, 8px radius, pixelated rendering. Below the inset, one line of the source Japanese sentence at 14px with the target word highlighted on a vermilion tint, and an Indonesian gloss beneath it in #6E6E73 at 13px.
Under the card, a centered row of four equal-width buttons: Again, Hard, Good, Easy. Good is filled vermilion with white semibold text; the other three are neutral white with hairline borders.
```

### 3.5 Screenshot hi-fi — companion / speaking practice

```
[STYLE BLOCK]

TASK: desktop application UI mockup, 1600x1000, speaking companion panel.
Layout: same slim 44px title bar, vermilion mark, centered title "Kana — speaking practice".
LEFT (flex): light gray stage #F5F5F7 with 36px padding holding a pixel-art GENERIC Japanese RPG scene (dusk sky band, a path, a single tree, one small character, and a white dialog box with two lines of small Japanese text). Along the bottom edge of the stage, a thin vermilion audio waveform strip, and beneath it a small caption row: "TTS · Kokoro ja" at the left.
RIGHT (520px, white, hairline left border): header row with a 36px square pixel-art avatar cropped from the game, the name "Kana" in semibold, and a tiny vermilion pill labeled "JA". Conversation body with three short bubbles: left-aligned white bubbles with hairline borders, right-aligned vermilion-tinted bubbles. One left bubble contains Japanese text with a small furigana hint, and an Indonesian gloss below it in #6E6E73 at 12.5px. Beneath the last bubble, an inline correction chip with a small vermilion check circle reading "gently corrected one particle · は → が" in #6E6E73.
Bottom row above a hairline border: a 40px circular vermilion microphone button at the left, a live vermilion waveform in the middle, and the caption "hold space to speak" at the right in #6E6E73 at 12px.
```

### 3.6 Brand sheet

```
[STYLE BLOCK]

TASK: a brand guideline sheet, 1600x1200, arranged as a clean typographic poster. No photography, no mockup scenes.
Top left: the app mark at 140px (speech bubble resolving into square pixels, vermilion #D9542F).
Top right block: the wordmark "OpenKotoba" at 88px semibold charcoal, with the tagline "LEARN JAPANESE BY PLAYING" letterspaced beneath it in #6E6E73, then two further letterspaced uppercase lines "100% OFFLINE" and "BUILT ON YOUR OWN GPU" in the same gray.
Middle band: six color swatches as 180x120 rectangles with hairline borders, each captioned beneath in small monospaced uppercase — WHITE FFFFFF, MIST F5F5F7, LINE E4E4E7, INK 1D1D1F, SLATE 6E6E73, VERMILION D9542F.
Bottom band: three type specimens labeled "DISPLAY 56 / 40 / 28", "BODY 15 / 13", "LABEL 11 UPPERCASE 0.08EM", each set at the size described, plus a row of four corner-radius squares.
Flat vector, crisp, no gradients, no shadows, no perspective, no extra decoration.
```

## 4. Yang harus diganti sebelum dipakai publik

1. **Nama.** "OpenKotoba" placeholder. `AGENTS.md` §7 melarang nama bermuatan "Emulingo", dan T6.7 belum menetapkan nama. Sebelum publik: cek bentrok di GitHub, crates.io, npm, dan daftar merek di negara target.
2. **Pixel art game.** Sekarang diagramatik. Ganti dengan screenshot asli (dengan izin lisensi atau tangkapan layar sendiri) setelah T2.4.
3. **Teks Jepang di mockup.** Sudah diperiksa: `このむらには ふるいでんせつが のこっている` dan `たびびとよ、こころして すすむがよい` adalah kalimat buatan sendiri, bukan kutipan dari game. Kalau nanti dipakai untuk marketing, kalimat yang dipakai harus dari screenshot nyata agar tidak menjanjikan output yang tidak dihasilkan model.
4. **Tabel kontras.** Vermilion di atas putih ≈ 3.6:1 — lolos untuk teks ≥ 18.66px bold dan untuk komponen UI, **gagal** untuk teks isi kecil. Verifikasi ulang dengan alat kontras sebelum rilis.

## 5. Prompt penutup untuk ChatGPT (opsional)

Kalau mau satu gambar "hero" untuk landing page (bukan mockup teknis), pakai ini:

```
[STYLE BLOCK]

TASK: a wide hero illustration, 2400x1200. A calm, near-white desk surface seen from directly above, flat vector, almost no detail. In the center a single open laptop drawn in outline, its screen filled with a soft vermilion glow and a faint grid of square pixels. Around it, four small objects spaced widely: a closed notebook, a pen, a pair of over-ear headphones, a coffee cup — all rendered as simple two-tone vector shapes in #F5F5F7 with #E4E4E7 outlines. Absolutely no text anywhere in the image, no logos, no people, no hands, no cables, no plant, no gradient mesh. Vast empty margins, lots of white space, poster-like restraint.
```