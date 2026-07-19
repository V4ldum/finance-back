-- Convert every table to STRICT typing. STRICT can only be set at CREATE TABLE
-- time (there is no ALTER TABLE ... SET STRICT), so we recreate each table with
-- the same table-rebuild dance used in the varchar_to_text migration.
--
-- PRAGMA foreign_keys=off cannot be used inside a transaction (sqlx wraps
-- migrations in one). Instead, new tables reference other _new tables to avoid
-- FK conflicts when dropping originals. SQLite auto-updates FK references when a
-- parent table is renamed (with foreign_keys=ON).
--
-- Two changes beyond adding STRICT:
--   * prices.date DATE -> TEXT. DATE is not a valid STRICT type name (only INT,
--     INTEGER, REAL, TEXT, BLOB, ANY are allowed). Dates were already stored as
--     ISO-8601 text; CAST guards against any numeric-coerced legacy value.
--   * prices and coin_assets become WITHOUT ROWID. Both have a non-integer /
--     composite primary key and small rows, so an index-organized layout removes
--     the redundant rowid index and makes primary-key lookups a single search.

-------------------------------------------------
-- 1. Create all new tables
--    FK references point to _new tables
-------------------------------------------------
CREATE TABLE prices_new
(
    name  TEXT not null
        primary key,
    value REAL not null,
    date  TEXT not null
) STRICT, WITHOUT ROWID;

CREATE TABLE coin_images_new
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    image_url     TEXT,
    thumbnail_url TEXT,
    lettering     TEXT,
    description   TEXT,
    copyright     TEXT
) STRICT;

CREATE TABLE coins_new
(
    id          INTEGER not null
        primary key autoincrement,
    numista_id  TEXT    not null,
    name        TEXT    not null,
    weight      REAL    not null,
    size        REAL    not null,
    thickness   REAL,
    min_year    TEXT    not null,
    max_year    TEXT,
    composition TEXT    not null,
    purity      INTEGER not null,
    obverse     INTEGER
        references coin_images_new(id),
    reverse     INTEGER
        references coin_images_new(id),
    edge        INTEGER
        references coin_images_new(id),
    check (composition IN ('GOLD', 'SILVER'))
) STRICT;

CREATE TABLE users_new
(
    id      INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    api_key TEXT UNIQUE NOT NULL
) STRICT;

CREATE TABLE coin_assets_new
(
    coin_id   INTEGER not null
        references coins_new(id),
    user_id   INTEGER not null
        references users_new(id),
    possessed INTEGER not null,
    primary key (coin_id, user_id)
) STRICT, WITHOUT ROWID;

CREATE TABLE raw_assets_new
(
    id          INTEGER not null
        primary key autoincrement,
    name        TEXT    not null,
    possessed   INTEGER not null,
    unit_weight INTEGER not null,
    composition TEXT    not null check (composition IN ('SILVER', 'GOLD')),
    purity      INTEGER not null,
    user_id     INTEGER not null,
    FOREIGN KEY (user_id) REFERENCES users_new(id)
) STRICT;

CREATE TABLE cash_assets_new
(
    id         INTEGER not null
        primary key autoincrement,
    name       TEXT    not null,
    possessed  INTEGER not null,
    unit_value INTEGER not null,
    user_id    INTEGER not null,
    FOREIGN KEY (user_id) REFERENCES users_new(id)
) STRICT;

-------------------------------------------------
-- 2. Copy all data (parents before children)
-------------------------------------------------
INSERT INTO prices_new (name, value, date)
    SELECT name, value, CAST(date AS TEXT) FROM prices;
INSERT INTO coin_images_new SELECT * FROM coin_images;
INSERT INTO users_new SELECT * FROM users;
INSERT INTO coins_new SELECT * FROM coins;
INSERT INTO coin_assets_new SELECT * FROM coin_assets;
INSERT INTO raw_assets_new SELECT * FROM raw_assets;
INSERT INTO cash_assets_new SELECT * FROM cash_assets;

-------------------------------------------------
-- 3. Drop original tables (children first)
-------------------------------------------------
DROP TABLE coin_assets;
DROP TABLE raw_assets;
DROP TABLE cash_assets;
DROP TABLE coins;
DROP TABLE users;
DROP TABLE coin_images;
DROP TABLE prices;

-------------------------------------------------
-- 4. Rename new tables (parents first)
--    SQLite auto-updates FK refs in child tables
-------------------------------------------------
ALTER TABLE prices_new RENAME TO prices;
ALTER TABLE coin_images_new RENAME TO coin_images;
ALTER TABLE users_new RENAME TO users;
ALTER TABLE coins_new RENAME TO coins;
ALTER TABLE coin_assets_new RENAME TO coin_assets;
ALTER TABLE raw_assets_new RENAME TO raw_assets;
ALTER TABLE cash_assets_new RENAME TO cash_assets;
