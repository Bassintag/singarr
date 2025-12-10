PRAGMA foreign_keys = ON;

-- Artist

CREATE TABLE artist (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "created_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    "name" TEXT NOT NULL,

    "lidarr_id" INTEGER UNIQUE,
    "musicbrainz_id" TEXT
);

-- Album

CREATE TABLE album (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "created_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    "title" TEXT NOT NULL,
    "release_date" DATE,
    "cover_path" TEXT,

    "lidarr_id" INTEGER UNIQUE,
    "musicbrainz_id" TEXT,
    "artist_id" INTEGER NOT NULL,

    FOREIGN KEY (artist_id) REFERENCES artist(id) ON DELETE CASCADE
);

CREATE INDEX idx_album_artist_id ON album(artist_id);

-- Track

CREATE TABLE track (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "created_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    "track_number" INTEGER,
    "title" TEXT NOT NULL,
    "file_path" TEXT NOT NULL UNIQUE,
    "duration_ms" INTEGER,

    "lidarr_id" INTEGER UNIQUE,
    "musicbrainz_id" TEXT,
    "album_id" INTEGER NOT NULL,
    "artist_id" INTEGER NOT NULL,

    FOREIGN KEY (album_id) REFERENCES album(id) ON DELETE CASCADE,
    FOREIGN KEY (artist_id) REFERENCES artist(id) ON DELETE CASCADE
);

CREATE INDEX idx_track_album_id ON track(album_id);
CREATE INDEX idx_track_artist_id ON track(artist_id);

-- LyricsFile

CREATE TABLE lyrics (

    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "created_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    "language" TEXT,
    "provider" TEXT,
    "synced" BOOLEAN NOT NULL,
    "file_path" TEXT NOT NULL UNIQUE,
    "checksum" TEXT NOT NULL,

    "track_id" INTEGER NOT NULL,

    FOREIGN KEY (track_id) REFERENCES track(id) ON DELETE CASCADE
);

CREATE INDEX idx_lyrics_track_id ON lyrics(track_id);
