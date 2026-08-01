-- Add down migration script here
DROP TRIGGER insert_coins_fts;
DROP TRIGGER delete_coins_fts;
DROP TRIGGER update_coins_fts;

DROP TABLE coins_fts;
