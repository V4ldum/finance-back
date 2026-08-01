-- Add up migration script here
CREATE VIRTUAL TABLE coins_fts USING fts5
(
    name,
    content = 'coins',
    content_rowid = 'id',
    tokenize = 'unicode61 remove_diacritics 2'
);

-- Update the virtual table on insert, delete & update
CREATE TRIGGER insert_coins_fts
AFTER INSERT ON coins
BEGIN
    INSERT INTO coins_fts(rowid, name)
    VALUES (new.id, new.name);
END;

CREATE TRIGGER delete_coins_fts
AFTER DELETE ON coins
BEGIN
    INSERT INTO coins_fts(coins_fts, rowid, name)
    VALUES ('delete', old.id, old.name);
END;

CREATE TRIGGER update_coins_fts
AFTER UPDATE OF name ON coins
BEGIN
    INSERT INTO coins_fts(coins_fts, rowid, name)
    VALUES ('delete', old.id, old.name);
    INSERT INTO coins_fts(rowid, name)
    VALUES (new.id, new.name);
END;

-- Backfill the virtual table with already existing data
INSERT INTO coins_fts(coins_fts)
VALUES ('rebuild');
