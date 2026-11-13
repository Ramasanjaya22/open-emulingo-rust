<script lang="ts">
    let gameTitle = "Pokémon Emerald (JA)";
    let serverStatus = "Online (127.0.0.1:8091)";
    let translationText = "ぜんこくずかん を 完成させよう！";
    let translationResult = "Let's complete the National Pokédex!";
    let vocabularyCount = 42;
</script>

<div class="min-h-screen bg-slate-50 text-slate-900 flex flex-col">
    <header class="border-b border-slate-200 bg-white px-6 py-4 flex items-center justify-between shadow-sm">
        <h1 class="text-xl font-semibold tracking-tight text-slate-800">OpenKotoba</h1>
        <div class="flex items-center gap-3 text-sm">
            <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-emerald-100 text-emerald-800">
                {serverStatus}
            </span>
        </div>
    </header>

    <main class="flex-1 max-w-6xl w-full mx-auto p-6 grid grid-cols-1 md:grid-cols-3 gap-6">
        <section class="md:col-span-2 bg-white rounded-xl shadow-sm border border-slate-200 p-6 flex flex-col">
            <h2 class="text-lg font-medium text-slate-800 mb-4">GBA Display & OCR Feed</h2>
            <div class="flex-1 bg-slate-900 rounded-lg flex items-center justify-center text-slate-400 min-h-[280px] font-mono">
                [GBA Frame Canvas - {gameTitle}]
            </div>
            <div class="mt-4 p-4 bg-slate-50 rounded-lg border border-slate-200">
                <div class="text-xs uppercase tracking-wider text-slate-500 mb-1">Detected Text</div>
                <div class="text-lg font-medium text-slate-800 mb-2">{translationText}</div>
                <div class="text-xs uppercase tracking-wider text-slate-500 mb-1">Translation</div>
                <div class="text-md text-emerald-700 font-medium">{translationResult}</div>
            </div>
        </section>

        <section class="bg-white rounded-xl shadow-sm border border-slate-200 p-6 flex flex-col justify-between">
            <div>
                <h2 class="text-lg font-medium text-slate-800 mb-4">Study Stats</h2>
                <div class="space-y-4">
                    <div class="p-4 bg-slate-50 rounded-lg border border-slate-200">
                        <div class="text-sm text-slate-500">Vocabulary Saved</div>
                        <div class="text-2xl font-semibold text-slate-800">{vocabularyCount}</div>
                    </div>
                    <div class="p-4 bg-slate-50 rounded-lg border border-slate-200">
                        <div class="text-sm text-slate-500">Active Model</div>
                        <div class="text-sm font-medium text-slate-800 mt-1">Qwen3.5-4B (Vulkan)</div>
                    </div>
                </div>
            </div>
            <button class="mt-6 w-full py-2.5 px-4 bg-slate-900 text-white rounded-lg font-medium hover:bg-slate-800 transition-colors">
                Translate Screen
            </button>
        </section>
    </main>
</div>
