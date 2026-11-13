<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { invoke, Channel } from '@tauri-apps/api/core';
    import { open } from '@tauri-apps/plugin-dialog';

    /// GBA screen size in pixels.
    const SCREEN_W = 240;
    const SCREEN_H = 160;
    /// Real GBA refresh rate, used only to smooth the FPS readout.
    const FPS_POLL_MS = 1000;

    interface RomSummary {
        sha256: string;
        title: string;
        game_code: string;
        lang: string | null;
        file_name: string;
    }

    interface RuntimeStatus {
        running: boolean;
        bios_found: boolean;
        fps: number;
    }

    let canvas: HTMLCanvasElement;
    let ctx: CanvasRenderingContext2D | null = null;
    let imageData: ImageData | null = null;

    let rom: RomSummary | null = null;
    let status: RuntimeStatus = { running: false, bios_found: false, fps: 0 };
    let message = 'Buka ROM GBA untuk mulai.';
    let busy = false;
    let showFps = false;

    /// Maps a keyboard key to a GBA button name.
    const KEY_MAP: Record<string, string> = {
        ArrowUp: 'Up',
        ArrowDown: 'Down',
        ArrowLeft: 'Left',
        ArrowRight: 'Right',
        z: 'A',
        Z: 'A',
        x: 'B',
        X: 'B',
        Enter: 'Start',
        Backspace: 'Select',
        a: 'L',
        A: 'L',
        s: 'R',
        S: 'R'
    };

    /// Draws one RGBA frame onto the canvas.
    function drawFrame(rgba: Uint8Array | number[]) {
        if (!ctx || !imageData) return;
        const bytes = rgba instanceof Uint8Array ? rgba : new Uint8Array(rgba);
        if (bytes.length !== SCREEN_W * SCREEN_H * 4) return;
        imageData.data.set(bytes);
        ctx.putImageData(imageData, 0, 0);
    }

    /// Opens the file picker, loads the ROM, and starts the frame stream.
    async function openRom() {
        busy = true;
        try {
            const picked = await open({
                multiple: false,
                filters: [{ name: 'GBA ROM', extensions: ['gba', 'agb', 'bin'] }]
            });
            if (!picked || typeof picked !== 'string') {
                message = 'Tidak ada ROM dipilih.';
                return;
            }

            // Frames arrive here, ~60 times per second, as raw RGBA.
            const onFrame = new Channel<number[]>();
            onFrame.onmessage = (data) => drawFrame(data);

            rom = await invoke<RomSummary>('load_rom', { path: picked, onFrame });
            message = `Bermain ${rom.title} (${rom.game_code})`;
            status = await invoke<RuntimeStatus>('runtime_status');
        } catch (err) {
            message = `Gagal memuat ROM: ${String(err)}`;
        } finally {
            busy = false;
        }
    }

    /// Forwards a key transition to the emulator.
    function sendKey(key: string, pressed: boolean) {
        if (!rom) return;
        void invoke('set_keys', { events: [{ key, pressed }] });
    }

    /// Keyboard handler: gameplay keys, plus F5/F9/F3 shortcuts.
    function onKeyDown(event: KeyboardEvent) {
        if (event.key === 'F5') {
            event.preventDefault();
            void saveState();
            return;
        }
        if (event.key === 'F9') {
            event.preventDefault();
            void loadState();
            return;
        }
        if (event.key === 'F3') {
            event.preventDefault();
            showFps = !showFps;
            return;
        }

        const mapped = KEY_MAP[event.key];
        if (!mapped) return;
        event.preventDefault();
        if (!event.repeat) sendKey(mapped, true);
    }

    /// Releases the button when the key comes up.
    function onKeyUp(event: KeyboardEvent) {
        const mapped = KEY_MAP[event.key];
        if (!mapped) return;
        event.preventDefault();
        sendKey(mapped, false);
    }

    /// Writes a save state and reports the path.
    async function saveState() {
        if (!rom) return;
        try {
            const path = await invoke<string>('save_state');
            message = `State disimpan: ${path}`;
        } catch (err) {
            message = `Gagal menyimpan state: ${String(err)}`;
        }
    }

    /// Restores the last save state.
    async function loadState() {
        if (!rom) return;
        try {
            await invoke('load_state');
            message = 'State dimuat.';
        } catch (err) {
            message = `Gagal memuat state: ${String(err)}`;
        }
    }

    /// Refreshes the FPS / BIOS badge once per second.
    let pollTimer: ReturnType<typeof setInterval> | undefined;

    onMount(() => {
        ctx = canvas.getContext('2d');
        if (ctx) {
            ctx.imageSmoothingEnabled = false;
            imageData = ctx.createImageData(SCREEN_W, SCREEN_H);
            // Start on a neutral grey so the empty screen is not pure black.
            const blank = new Uint8Array(SCREEN_W * SCREEN_H * 4).fill(0xf0);
            for (let i = 3; i < blank.length; i += 4) blank[i] = 0xff;
            drawFrame(blank);
        }

        window.addEventListener('keydown', onKeyDown);
        window.addEventListener('keyup', onKeyUp);

        pollTimer = setInterval(() => {
            if (!rom) return;
            void invoke<RuntimeStatus>('runtime_status').then((next) => {
                status = next;
            });
        }, FPS_POLL_MS);
    });

    onDestroy(() => {
        window.removeEventListener('keydown', onKeyDown);
        window.removeEventListener('keyup', onKeyUp);
        if (pollTimer) clearInterval(pollTimer);
        void invoke('stop_emulator').catch(() => {});
    });
</script>

<div class="app">
    <header>
        <h1>OpenKotoba</h1>
        <div class="actions">
            <span class="badge" class:ok={status.bios_found} class:warn={!status.bios_found}>
                {status.bios_found ? 'BIOS siap' : 'BIOS tidak ditemukan'}
            </span>
            {#if showFps}
                <span class="badge">{status.fps.toFixed(1)} fps</span>
            {/if}
            <button class="primary" on:click={openRom} disabled={busy}>
                {busy ? 'Memuat…' : 'Buka ROM…'}
            </button>
        </div>
    </header>

    <main>
        <section class="screen-panel">
            <div class="screen-frame">
                <canvas width={SCREEN_W} height={SCREEN_H} bind:this={canvas}></canvas>
            </div>
            <p class="message">{message}</p>
        </section>

        <aside>
            <h2>Game</h2>
            {#if rom}
                <dl>
                    <dt>Judul</dt>
                    <dd>{rom.title}</dd>
                    <dt>Kode</dt>
                    <dd>{rom.game_code}</dd>
                    <dt>Bahasa</dt>
                    <dd>{rom.lang === 'ja' ? 'Jepang' : rom.lang === 'en' ? 'Inggris' : 'Tidak diketahui'}</dd>
                    <dt>Berkas</dt>
                    <dd class="wrap">{rom.file_name}</dd>
                </dl>
            {:else}
                <p class="muted">Belum ada ROM dimuat.</p>
            {/if}

            <h2>Kontrol</h2>
            <ul class="keys">
                <li><kbd>←↑↓→</kbd><span>D-pad</span></li>
                <li><kbd>Z</kbd><span>A</span></li>
                <li><kbd>X</kbd><span>B</span></li>
                <li><kbd>Enter</kbd><span>Start</span></li>
                <li><kbd>Backspace</kbd><span>Select</span></li>
                <li><kbd>A</kbd><kbd>S</kbd><span>L / R</span></li>
                <li><kbd>F5</kbd><kbd>F9</kbd><span>Simpan / muat state</span></li>
                <li><kbd>F3</kbd><span>FPS</span></li>
            </ul>

            <div class="row">
                <button on:click={saveState} disabled={!rom}>Simpan state</button>
                <button on:click={loadState} disabled={!rom}>Muat state</button>
            </div>
        </aside>
    </main>
</div>

<style>
    .app {
        display: flex;
        flex-direction: column;
        height: 100vh;
    }

    header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 16px;
        padding: 14px 22px;
        background: var(--surface);
        border-bottom: 1px solid var(--border);
    }

    h1 {
        font-size: 17px;
    }

    .actions {
        display: flex;
        align-items: center;
        gap: 10px;
    }

    .badge {
        font-size: 12px;
        color: var(--text-muted);
        border: 1px solid var(--border);
        border-radius: 999px;
        padding: 3px 10px;
        white-space: nowrap;
    }

    .badge.ok {
        color: var(--accent);
        background: var(--accent-soft);
        border-color: #cfe6da;
    }

    .badge.warn {
        color: #8a5a00;
        background: #fdf4e3;
        border-color: #f0dfbc;
    }

    main {
        flex: 1;
        display: grid;
        grid-template-columns: minmax(0, 1fr) 280px;
        gap: 20px;
        padding: 20px 22px;
        min-height: 0;
    }

    .screen-panel {
        display: flex;
        flex-direction: column;
        min-height: 0;
        min-width: 0;
    }

    .screen-frame {
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
        background: var(--surface);
        border: 1px solid var(--border);
        border-radius: var(--radius);
        box-shadow: var(--shadow);
        padding: 16px;
        min-height: 0;
        overflow: hidden;
    }

    canvas {
        /* Integer scaling keeps the pixels square and crisp. */
        width: min(100%, calc((100vh - 190px) * 1.5));
        aspect-ratio: 3 / 2;
        height: auto;
        border-radius: 4px;
        background: #202024;
    }

    .message {
        margin: 12px 2px 0;
        font-size: 13px;
        color: var(--text-muted);
        min-height: 20px;
    }

    aside {
        display: flex;
        flex-direction: column;
        gap: 12px;
        background: var(--surface);
        border: 1px solid var(--border);
        border-radius: var(--radius);
        box-shadow: var(--shadow);
        padding: 18px;
        overflow-y: auto;
    }

    aside h2 {
        font-size: 13px;
        text-transform: uppercase;
        letter-spacing: 0.06em;
        color: var(--text-muted);
        margin-top: 6px;
    }

    dl {
        margin: 0;
        display: grid;
        grid-template-columns: auto 1fr;
        gap: 4px 12px;
        font-size: 13px;
    }

    dt {
        color: var(--text-muted);
    }

    dd {
        margin: 0;
        font-weight: 500;
    }

    dd.wrap {
        overflow-wrap: anywhere;
    }

    .keys {
        list-style: none;
        margin: 0;
        padding: 0;
        display: grid;
        gap: 6px;
        font-size: 13px;
    }

    .keys li {
        display: flex;
        align-items: center;
        gap: 6px;
    }

    .keys li span {
        margin-left: auto;
        color: var(--text-muted);
    }

    kbd {
        font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
        font-size: 11px;
        background: #f5f5f7;
        border: 1px solid var(--border);
        border-bottom-width: 2px;
        border-radius: 5px;
        padding: 1px 5px;
        min-width: 20px;
        text-align: center;
    }

    .row {
        display: flex;
        gap: 8px;
        margin-top: auto;
    }

    .row button {
        flex: 1;
        font-size: 13px;
        padding: 7px 10px;
    }
</style>
