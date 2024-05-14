CREATE TABLE IF NOT EXISTS "prices"
(
    name  VARCHAR(10) not null
        primary key,
    value REAL        not null,
    date  DATE        not null
);
CREATE TABLE sqlite_sequence(name,seq);
CREATE TABLE coin_images(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, 
    image_url VARCHAR(100), 
    thumbnail_url VARCHAR(100), 
    lettering VARCHAR(100), 
    description VARCHAR(100), 
    copyright VARCHAR(100)
);
CREATE TABLE IF NOT EXISTS "coins"
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
        references coin_images(id),
    reverse     INTEGER
        references coin_images(id),
    edge        INTEGER
        references coin_images(id),
    check (composition IN ('GOLD', 'SILVER'))
);
CREATE TABLE users(
id integer primary key autoincrement not null,
api_key varchar(30) unique not null
);
CREATE TABLE IF NOT EXISTS "coin_assets"
(
    coin_id   INTEGER not null
        references coins(id),
    user_id   INTEGER not null
        references users(id),
    possessed INTEGER not null,
    primary key (coin_id, user_id)
);
CREATE TABLE IF NOT EXISTS "raw_assets"
(
    id          INTEGER     not null
        primary key autoincrement,
    name        VARCHAR(50) not null,
    possessed   INTEGER     not null,
    unit_weight INTEGER     not null,
    composition VARCHAR(6)  not null check (composition IN ('SILVER', 'GOLD')),
    purity      INTEGER     not null,
    id_user     INTEGER     not null,
    FOREIGN KEY (id_user) REFERENCES users(id)
);
CREATE TABLE IF NOT EXISTS "cash_assets"
(
    id         INTEGER     not null
        primary key autoincrement,
    name       VARCHAR(50) not null,
    possessed  INTEGER     not null,
    unit_value INTEGER     not null,
    id_user    INTEGER     not null,
    FOREIGN KEY (id_user) REFERENCES users(id)
);
