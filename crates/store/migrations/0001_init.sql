-- Schema v1 for OpenKotoba (open-emulingo-rust)
CREATE TABLE IF NOT EXISTS games (
    sha256 BLOB PRIMARY KEY,
    title TEXT NOT NULL,
    game_code TEXT NOT NULL,
    lang TEXT,
    added_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS regions (
    game_code TEXT NOT NULL,
    name TEXT NOT NULL,
    x INTEGER NOT NULL,
    y INTEGER NOT NULL,
    w INTEGER NOT NULL,
    h INTEGER NOT NULL,
    PRIMARY KEY(game_code, name)
);

CREATE TABLE IF NOT EXISTS translations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    game_code TEXT NOT NULL,
    region_hash INTEGER NOT NULL,
    target_lang TEXT NOT NULL,
    source_lang TEXT NOT NULL,
    json TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(game_code, region_hash, target_lang)
);

CREATE TABLE IF NOT EXISTS lexicon (
    lemma TEXT NOT NULL,
    lang TEXT NOT NULL,
    target_lang TEXT NOT NULL,
    gloss TEXT NOT NULL,
    source TEXT CHECK(source IN('jmdict','kaikki','llm')) NOT NULL,
    PRIMARY KEY(lemma, lang, target_lang)
);

CREATE TABLE IF NOT EXISTS cards (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    lemma TEXT NOT NULL,
    lang TEXT NOT NULL,
    example TEXT,
    image BLOB,
    fsrs_state TEXT,
    due_at DATETIME
);

CREATE TABLE IF NOT EXISTS reviews (
    card_id INTEGER NOT NULL,
    rating INTEGER NOT NULL,
    reviewed_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(card_id) REFERENCES cards(id)
);

CREATE TABLE IF NOT EXISTS companions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    persona TEXT NOT NULL,
    avatar BLOB,
    lang TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS memories (
    companion_id INTEGER NOT NULL,
    fact TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(companion_id) REFERENCES companions(id)
);

CREATE TABLE IF NOT EXISTS stats (
    day TEXT PRIMARY KEY,
    xp INTEGER DEFAULT 0,
    reviews INTEGER DEFAULT 0,
    translations INTEGER DEFAULT 0
);

CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
