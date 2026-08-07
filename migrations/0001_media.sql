BEGIN;
-- setup

-- ===============================================
-- Root entities
-- ===============================================

CREATE TABLE media (
    id          SERIAL PRIMARY KEY,

    title       TEXT NOT NULL,
    synopsis    TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ===============================================
-- Basic Properties
-- ===============================================

-- Any audiovisual work
CREATE TABLE audiovisual (
    media_id INTEGER PRIMARY KEY
        REFERENCES media(id)
        ON DELETE CASCADE,
);

-- Any printed work
CREATE TABLE print (
    media_id INTEGER PRIMARY KEY
        REFERENCES media(id)
        ON DELETE CASCADE
);

-- Animation-specific data
CREATE TABLE animation (
    media_id INTEGER PRIMARY KEY
        REFERENCES audiovisual(media_id)
        ON DELETE CASCADE,

    studio TEXT
);

-- ===============================================
-- Basic types
-- ===============================================

-- Books
CREATE TABLE book (
    media_id INTEGER PRIMARY KEY
        REFERENCES print(media_id)
        ON DELETE CASCADE,
    
    isbn VARCHAR(13) UNIQUE,
    author TEXT,
    page_count INTEGER
);


-- Movies
CREATE TABLE movie (
    media_id INTEGER PRIMARY KEY
        REFERENCES audiovisual(media_id)
        ON DELETE CASCADE,

    runtime_minutes INTEGER NOT NULL
);

-- Episodic shows
CREATE TABLE show (
    media_id INTEGER PRIMARY KEY
        REFERENCES audiovisual(media_id)
        ON DELETE CASCADE,

    episodes_count INTEGER
);

-- ===============================================
-- Composit types
-- ===============================================

-- comic
CREATE TABLE comic (
    media_id INTEGER PRIMARY KEY
        REFERENCES book(media_id)
        ON DELETE CASCADE,

    artists TEXT[]
);

-- Japanese comics
CREATE TABLE manga (
    media_id INTEGER PRIMARY KEY
        REFERENCES comic(media_id)
        ON DELETE CASCADE,

    demographic TEXT
);

COMMIT;

