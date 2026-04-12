-- Revert TEXT columns back to VARCHAR(N).

-------------------------------------------------
-- 1. Create all new tables
-------------------------------------------------
CREATE TABLE prices_new
(
    name  VARCHAR(10) not null
        primary key,
    value REAL        not null,
    date  DATE        not null
);

CREATE TABLE coin_images_new
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    image_url     VARCHAR(100),
    thumbnail_url VARCHAR(100),
    lettering     VARCHAR(100),
    description   VARCHAR(100),
    copyright     VARCHAR(100)
);

CREATE TABLE coins_new
(
    id          INTEGER      not null
        primary key autoincrement,
    numista_id  VARCHAR(10)  not null,
    name        VARCHAR(100) not null,
    weight      REAL         not null,
    size        REAL         not null,
    thickness   REAL,
    min_year    VARCHAR(4)   not null,
    max_year    VARCHAR(4),
    composition VARCHAR(10)  not null,
    purity      INTEGER      not null,
    obverse     INTEGER
        references coin_images_new(id),
    reverse     INTEGER
        references coin_images_new(id),
    edge        INTEGER
        references coin_images_new(id),
    check (composition IN ('GOLD', 'SILVER'))
);

CREATE TABLE users_new
(
    id      INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    api_key VARCHAR(30) UNIQUE NOT NULL
);

CREATE TABLE coin_assets_new
(
    coin_id   INTEGER not null
        references coins_new(id),
    user_id   INTEGER not null
        references users_new(id),
    possessed INTEGER not null,
    primary key (coin_id, user_id)
);

CREATE TABLE raw_assets_new
(
    id          INTEGER     not null
        primary key autoincrement,
    name        VARCHAR(50) not null,
    possessed   INTEGER     not null,
    unit_weight INTEGER     not null,
    composition VARCHAR(6)  not null check (composition IN ('SILVER', 'GOLD')),
    purity      INTEGER     not null,
    id_user     INTEGER     not null,
    FOREIGN KEY (id_user) REFERENCES users_new(id)
);

CREATE TABLE cash_assets_new
(
    id         INTEGER     not null
        primary key autoincrement,
    name       VARCHAR(50) not null,
    possessed  INTEGER     not null,
    unit_value INTEGER     not null,
    id_user    INTEGER     not null,
    FOREIGN KEY (id_user) REFERENCES users_new(id)
);

-------------------------------------------------
-- 2. Copy all data (parents before children)
-------------------------------------------------
INSERT INTO prices_new SELECT * FROM prices;
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
-------------------------------------------------
ALTER TABLE prices_new RENAME TO prices;
ALTER TABLE coin_images_new RENAME TO coin_images;
ALTER TABLE users_new RENAME TO users;
ALTER TABLE coins_new RENAME TO coins;
ALTER TABLE coin_assets_new RENAME TO coin_assets;
ALTER TABLE raw_assets_new RENAME TO raw_assets;
ALTER TABLE cash_assets_new RENAME TO cash_assets;
